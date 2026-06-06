package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"github.com/algorand/go-algorand/cmd/util/datadir"
	v2 "github.com/algorand/go-algorand/daemon/algod/api/server/v2"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/ledger/simulation"
	"github.com/algorand/go-algorand/protocol"
)

var (
	safetyTxFilename              string
	safetyPolicyFilename          string
	safetyOutFilename             string
	safetyRound                   basics.Round
	safetyAllowEmptySignatures    bool
	safetyAllowMoreLogging        bool
	safetyAllowUnnamedResources   bool
	safetyAllowMoreOpcodeBudget   bool
	safetyExtraOpcodeBudget       int
	safetyIncludeSimulationResult bool
)

type safetyPolicy struct {
	MaxAlgoOutflow            *uint64           `json:"max-algo-outflow,omitempty"`
	MaxAssetOutflow           map[uint64]uint64 `json:"max-asset-outflow,omitempty"`
	EnforcementMode           string            `json:"enforcement-mode,omitempty"`
	AllowRekey                *bool             `json:"allow-rekey,omitempty"`
	AllowedReceivers          []string          `json:"allowed-receivers,omitempty"`
	BlockedReceivers          []string          `json:"blocked-receivers,omitempty"`
	AllowedAppIDs             []uint64          `json:"allowed-app-ids,omitempty"`
	BlockedAppIDs             []uint64          `json:"blocked-app-ids,omitempty"`
	AllowAppGlobalDeletes     *bool             `json:"allow-app-global-deletes,omitempty"`
	MaxInnerTxns              *uint32           `json:"max-inner-txns,omitempty"`
	HighRiskSignals           []string          `json:"high-risk-signals,omitempty"`
	AllowBypassIfUnavailable  *bool             `json:"allow-bypass-if-unavailable,omitempty"`
	RequiredRiskSignalsAbsent []string          `json:"required-risk-signals-absent,omitempty"`
}

type safetySimulateOutput struct {
	OutcomeSummary     v2.SafetyOutcomeSummary        `json:"outcome-summary"`
	OutcomeHash        string                         `json:"outcome-hash"`
	RiskSignals        []string                       `json:"risk-signals"`
	RiskLevel          string                         `json:"risk-level,omitempty"`
	VerificationStatus string                         `json:"verification-status,omitempty"`
	UnavailableReason  string                         `json:"unavailable-reason,omitempty"`
	Simulation         *v2.PreEncodedSimulateResponse `json:"simulation,omitempty"`
}

type safetyEvaluateOutput struct {
	Allowed            bool                    `json:"allowed"`
	Violations         []v2.SafetyViolation    `json:"violations"`
	Decision           string                  `json:"decision"`
	HardStop           bool                    `json:"hard-stop"`
	CanBypass          bool                    `json:"can-bypass"`
	EnforcementMode    string                  `json:"enforcement-mode"`
	VerificationStatus string                  `json:"verification-status,omitempty"`
	UnavailableReason  string                  `json:"unavailable-reason,omitempty"`
	OutcomeSummary     v2.SafetyOutcomeSummary `json:"outcome-summary"`
	OutcomeHash        string                  `json:"outcome-hash"`
}

func init() {
	clerkCmd.AddCommand(safetyCmd)
	safetyCmd.AddCommand(safetySimulateCmd)
	safetyCmd.AddCommand(safetyEvaluateCmd)

	addSafetySimulationFlags(safetySimulateCmd)
	safetySimulateCmd.Flags().StringVarP(&safetyOutFilename, "out", "o", "", "Write safety output JSON to file")
	safetySimulateCmd.Flags().BoolVar(&safetyIncludeSimulationResult, "include-simulation", false, "Include raw simulation payload in output")

	addSafetySimulationFlags(safetyEvaluateCmd)
	safetyEvaluateCmd.Flags().StringVarP(&safetyPolicyFilename, "policy", "p", "", "JSON file containing safety policy")
	safetyEvaluateCmd.Flags().StringVarP(&safetyOutFilename, "out", "o", "", "Write safety evaluation JSON to file")
	safetyEvaluateCmd.MarkFlagRequired("policy")
}

func addSafetySimulationFlags(cmd *cobra.Command) {
	cmd.Flags().StringVarP(&safetyTxFilename, "txfile", "t", "", "Transaction or transaction-group file")
	cmd.Flags().Uint64Var((*uint64)(&safetyRound), "round", 0, "Round to simulate against (defaults to latest)")
	cmd.Flags().BoolVar(&safetyAllowEmptySignatures, "allow-empty-signatures", false, "Allow unsigned transactions in simulation")
	cmd.Flags().BoolVar(&safetyAllowMoreLogging, "allow-more-logging", false, "Lift simulation log limits")
	cmd.Flags().BoolVar(&safetyAllowUnnamedResources, "allow-unnamed-resources", false, "Allow unnamed resources during simulation")
	cmd.Flags().BoolVar(&safetyAllowMoreOpcodeBudget, "allow-more-opcode-budget", false, "Use max extra opcode budget during simulation")
	cmd.Flags().IntVar(&safetyExtraOpcodeBudget, "extra-opcode-budget", 0, "Apply extra opcode budget during simulation")
	cmd.MarkFlagRequired("txfile")
}

var safetyCmd = &cobra.Command{
	Use:   "safety",
	Short: "Safety analysis commands for transaction groups",
	Args:  validateNoPosArgsFn,
	Run: func(cmd *cobra.Command, args []string) {
		cmd.HelpFunc()(cmd, args)
	},
}

var safetySimulateCmd = &cobra.Command{
	Use:   "simulate",
	Short: "Simulate a transaction group and emit deterministic safety summary",
	Args:  validateNoPosArgsFn,
	Run: func(cmd *cobra.Command, args []string) {
		response := simulateSafetyRequest()
		out := safetySimulateOutput{
			OutcomeSummary:     response.OutcomeSummary,
			OutcomeHash:        response.OutcomeHash,
			RiskSignals:        response.RiskSignals,
			RiskLevel:          response.RiskLevel,
			VerificationStatus: response.VerificationStatus,
			UnavailableReason:  response.UnavailableReason,
			Simulation:         response.Simulation,
		}
		emitSafetyOutput(out)
	},
}

var safetyEvaluateCmd = &cobra.Command{
	Use:   "evaluate",
	Short: "Evaluate a safety policy against a simulated transaction group",
	Args:  validateNoPosArgsFn,
	Run: func(cmd *cobra.Command, args []string) {
		policy := loadSafetyPolicyV2(safetyPolicyFilename)
		request := buildSafetyEvaluateRequest(policy)
		client := ensureFullClient(datadir.EnsureSingleDataDir())
		response, err := client.EvaluateSafety(request)
		if err != nil {
			reportErrorf("safety evaluation error: %s", err)
		}

		out := safetyEvaluateOutput{
			Allowed:            response.Allowed,
			Violations:         response.Violations,
			Decision:           response.Decision,
			HardStop:           response.HardStop,
			CanBypass:          response.CanBypass,
			EnforcementMode:    response.EnforcementMode,
			VerificationStatus: response.VerificationStatus,
			UnavailableReason:  response.UnavailableReason,
			OutcomeSummary:     response.OutcomeSummary,
			OutcomeHash:        response.OutcomeHash,
		}
		emitSafetyOutput(out)
	},
}

func simulateSafetyRequest() v2.PreEncodedSafetySimulateResponse {
	if safetyAllowMoreOpcodeBudget && safetyExtraOpcodeBudget != 0 {
		reportErrorf("--allow-more-opcode-budget and --extra-opcode-budget are mutually exclusive")
	}
	extraBudget := safetyExtraOpcodeBudget
	if safetyAllowMoreOpcodeBudget {
		extraBudget = simulation.MaxExtraOpcodeBudget
	}

	txgroup := decodeTxnsFromFile(safetyTxFilename)
	request := v2.PreEncodedSimulateRequest{
		TxnGroups:             []v2.PreEncodedSimulateRequestTransactionGroup{{Txns: txgroup}},
		Round:                 safetyRound,
		AllowEmptySignatures:  safetyAllowEmptySignatures,
		AllowMoreLogging:      safetyAllowMoreLogging,
		AllowUnnamedResources: safetyAllowUnnamedResources,
		ExtraOpcodeBudget:     extraBudget,
	}

	safetyRequest := v2.PreEncodedSafetySimulateRequest{
		SimulateRequest:   request,
		IncludeSimulation: safetyIncludeSimulationResult,
	}

	client := ensureFullClient(datadir.EnsureSingleDataDir())
	response, err := client.SimulateSafety(safetyRequest)
	if err != nil {
		reportErrorf("safety simulation error: %s", err)
	}
	return response
}

func buildSafetyEvaluateRequest(policy v2.SafetyPolicy) v2.PreEncodedSafetyEvaluateRequest {
	if safetyAllowMoreOpcodeBudget && safetyExtraOpcodeBudget != 0 {
		reportErrorf("--allow-more-opcode-budget and --extra-opcode-budget are mutually exclusive")
	}
	extraBudget := safetyExtraOpcodeBudget
	if safetyAllowMoreOpcodeBudget {
		extraBudget = simulation.MaxExtraOpcodeBudget
	}

	txgroup := decodeTxnsFromFile(safetyTxFilename)
	return v2.PreEncodedSafetyEvaluateRequest{
		SimulateRequest: v2.PreEncodedSimulateRequest{
			TxnGroups:             []v2.PreEncodedSimulateRequestTransactionGroup{{Txns: txgroup}},
			Round:                 safetyRound,
			AllowEmptySignatures:  safetyAllowEmptySignatures,
			AllowMoreLogging:      safetyAllowMoreLogging,
			AllowUnnamedResources: safetyAllowUnnamedResources,
			ExtraOpcodeBudget:     extraBudget,
		},
		Policy: policy,
	}
}

func emitSafetyOutput(payload any) {
	encoded := protocol.EncodeJSON(payload)
	if safetyOutFilename != "" {
		err := writeFile(safetyOutFilename, encoded, 0600)
		if err != nil {
			reportErrorf("write file error: %s", err)
		}
		return
	}
	fmt.Println(string(encoded))
}

func loadSafetyPolicy(filename string) safetyPolicy {
	data, err := os.ReadFile(filename)
	if err != nil {
		reportErrorf(fileReadError, filename, err)
	}
	var policy safetyPolicy
	if err := json.Unmarshal(data, &policy); err != nil {
		reportErrorf("unable to decode policy file %s: %v", filename, err)
	}
	return policy
}

func loadSafetyPolicyV2(filename string) v2.SafetyPolicy {
	policy := loadSafetyPolicy(filename)
	return v2.SafetyPolicy{
		MaxAlgoOutflow:            policy.MaxAlgoOutflow,
		MaxAssetOutflow:           policy.MaxAssetOutflow,
		EnforcementMode:           policy.EnforcementMode,
		AllowRekey:                policy.AllowRekey,
		AllowedReceivers:          policy.AllowedReceivers,
		BlockedReceivers:          policy.BlockedReceivers,
		AllowedAppIDs:             policy.AllowedAppIDs,
		BlockedAppIDs:             policy.BlockedAppIDs,
		AllowAppGlobalDeletes:     policy.AllowAppGlobalDeletes,
		MaxInnerTxns:              policy.MaxInnerTxns,
		HighRiskSignals:           policy.HighRiskSignals,
		AllowBypassIfUnavailable:  policy.AllowBypassIfUnavailable,
		RequiredRiskSignalsAbsent: policy.RequiredRiskSignalsAbsent,
	}
}
