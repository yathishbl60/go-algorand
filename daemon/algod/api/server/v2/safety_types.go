package v2

import (
	"github.com/algorand/go-algorand/data/basics"
)

// PreEncodedSafetySimulateRequest wraps a simulation request and endpoint-specific options.
type PreEncodedSafetySimulateRequest struct {
	SimulateRequest   PreEncodedSimulateRequest `codec:"simulate-request"`
	IncludeSimulation bool                      `codec:"include-simulation,omitempty"`
}

// SafetyPolicy defines user-provided guardrails evaluated against predicted outcomes.
type SafetyPolicy struct {
	MaxAlgoOutflow            *uint64           `codec:"max-algo-outflow,omitempty"`
	MaxAssetOutflow           map[uint64]uint64 `codec:"max-asset-outflow,omitempty"`
	EnforcementMode           string            `codec:"enforcement-mode,omitempty"`
	AllowRekey                *bool             `codec:"allow-rekey,omitempty"`
	AllowedReceivers          []string          `codec:"allowed-receivers,omitempty"`
	BlockedReceivers          []string          `codec:"blocked-receivers,omitempty"`
	AllowedAppIDs             []uint64          `codec:"allowed-app-ids,omitempty"`
	BlockedAppIDs             []uint64          `codec:"blocked-app-ids,omitempty"`
	AllowAppGlobalDeletes     *bool             `codec:"allow-app-global-deletes,omitempty"`
	MaxInnerTxns              *uint32           `codec:"max-inner-txns,omitempty"`
	HighRiskSignals           []string          `codec:"high-risk-signals,omitempty"`
	AllowBypassIfUnavailable  *bool             `codec:"allow-bypass-if-unavailable,omitempty"`
	RequiredRiskSignalsAbsent []string          `codec:"required-risk-signals-absent,omitempty"`
}

// PreEncodedSafetyEvaluateRequest evaluates policy against a simulate request.
type PreEncodedSafetyEvaluateRequest struct {
	SimulateRequest PreEncodedSimulateRequest `codec:"simulate-request"`
	Policy          SafetyPolicy              `codec:"policy"`
}

// SafetyAlgoDelta captures net Algo change for an address.
type SafetyAlgoDelta struct {
	Address string `codec:"address"`
	Delta   int64  `codec:"delta"`
}

// SafetyAssetDelta captures net ASA change for an address and asset.
type SafetyAssetDelta struct {
	Address string `codec:"address"`
	AssetID uint64 `codec:"asset-id"`
	Delta   int64  `codec:"delta"`
}

// SafetyAlgoTransfer captures a concrete Algo transfer effect.
type SafetyAlgoTransfer struct {
	From   string `codec:"from"`
	To     string `codec:"to"`
	Amount uint64 `codec:"amount"`
	Kind   string `codec:"kind"`
}

// SafetyAssetTransfer captures a concrete ASA transfer effect.
type SafetyAssetTransfer struct {
	AssetID uint64 `codec:"asset-id"`
	From    string `codec:"from"`
	To      string `codec:"to"`
	Amount  uint64 `codec:"amount"`
	Kind    string `codec:"kind"`
}

// SafetyAuthChange captures a rekey/authentication change effect.
type SafetyAuthChange struct {
	Address string `codec:"address"`
	RekeyTo string `codec:"rekey-to"`
}

// SafetyAppStateChange captures deterministic app state mutations.
type SafetyAppStateChange struct {
	AppID   uint64 `codec:"app-id"`
	Scope   string `codec:"scope"`
	Account string `codec:"account,omitempty"`
	KeyB64  string `codec:"key-b64"`
	Action  uint64 `codec:"action"`
}

// SafetyResourceLifecycle captures resource create/delete/reconfigure effects.
type SafetyResourceLifecycle struct {
	Kind    string `codec:"kind"`
	Action  string `codec:"action"`
	ID      uint64 `codec:"id,omitempty"`
	Address string `codec:"address,omitempty"`
}

// SafetyOutcomeSummary captures deterministic predicted effects for a transaction group.
type SafetyOutcomeSummary struct {
	Round             basics.Round              `codec:"round"`
	GroupID           string                    `codec:"group-id,omitempty"`
	AlgoDeltas        []SafetyAlgoDelta         `codec:"algo-deltas"`
	AssetDeltas       []SafetyAssetDelta        `codec:"asset-deltas"`
	AlgoTransfers     []SafetyAlgoTransfer      `codec:"algo-transfers"`
	AssetTransfers    []SafetyAssetTransfer     `codec:"asset-transfers"`
	AppStateChanges   []SafetyAppStateChange    `codec:"app-state-changes"`
	AuthChanges       []SafetyAuthChange        `codec:"auth-changes"`
	ResourceLifecycle []SafetyResourceLifecycle `codec:"resource-lifecycle"`
	AppIDsTouched     []uint64                  `codec:"app-ids-touched"`
	InnerTxnCount     uint64                    `codec:"inner-txn-count"`
	RiskSignals       []string                  `codec:"risk-signals"`
	HumanSummary      []string                  `codec:"human-summary,omitempty"`
}

// SafetyViolation describes a deterministic policy violation.
type SafetyViolation struct {
	Code     string `codec:"code"`
	Path     string `codec:"path"`
	Message  string `codec:"message"`
	Severity string `codec:"severity"`
}

// Safety verification and policy decision constants returned by safety endpoints.
const (
	SafetyVerificationVerified    = "verified"
	SafetyVerificationUnavailable = "unavailable"

	SafetyEnforcementSoft = "soft"
	SafetyEnforcementHard = "hard"

	SafetyDecisionAllow      = "allow"
	SafetyDecisionWarn       = "warn"
	SafetyDecisionBlock      = "block"
	SafetyDecisionUnverified = "unverified"
)

// PreEncodedSafetySimulateResponse is returned by safety simulation endpoint.
type PreEncodedSafetySimulateResponse struct {
	OutcomeSummary     SafetyOutcomeSummary        `codec:"outcome-summary"`
	OutcomeHash        string                      `codec:"outcome-hash"`
	RiskSignals        []string                    `codec:"risk-signals"`
	RiskLevel          string                      `codec:"risk-level,omitempty"`
	VerificationStatus string                      `codec:"verification-status,omitempty"`
	UnavailableReason  string                      `codec:"unavailable-reason,omitempty"`
	Simulation         *PreEncodedSimulateResponse `codec:"simulation,omitempty"`
}

// PreEncodedSafetyEvaluateResponse is returned by safety policy evaluation endpoint.
type PreEncodedSafetyEvaluateResponse struct {
	Allowed            bool                 `codec:"allowed"`
	Violations         []SafetyViolation    `codec:"violations"`
	Decision           string               `codec:"decision"`
	HardStop           bool                 `codec:"hard-stop"`
	CanBypass          bool                 `codec:"can-bypass"`
	EnforcementMode    string               `codec:"enforcement-mode"`
	VerificationStatus string               `codec:"verification-status,omitempty"`
	UnavailableReason  string               `codec:"unavailable-reason,omitempty"`
	OutcomeSummary     SafetyOutcomeSummary `codec:"outcome-summary"`
	OutcomeHash        string               `codec:"outcome-hash"`
}
