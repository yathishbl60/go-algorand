package v2

import (
	"fmt"
	"slices"
	"strings"

	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

type safetyAssetDeltaKey struct {
	Address string
	AssetID uint64
}

type safetyAccumulator struct {
	algoDeltas        map[string]int64
	assetDeltas       map[safetyAssetDeltaKey]int64
	algoTransfers     []SafetyAlgoTransfer
	assetTransfers    []SafetyAssetTransfer
	appStateChanges   []SafetyAppStateChange
	authChanges       map[string]string
	resourceLifecycle []SafetyResourceLifecycle
	appIDsTouched     map[uint64]struct{}
	innerTxnCount     uint64
	riskSignals       map[string]struct{}
}

func buildSafetyOutcomeSummary(response PreEncodedSimulateResponse) SafetyOutcomeSummary {
	acc := newSafetyAccumulator()
	accumulateSummaryFromSimulation(&acc, response)
	summary := newSafetySummary(response.LastRound, &acc)
	appendSummaryMapValues(&summary, &acc)
	sortSafetySummary(&summary)
	summary.HumanSummary = buildHumanSummary(summary)
	return summary
}

func newSafetyAccumulator() safetyAccumulator {
	return safetyAccumulator{
		algoDeltas:    make(map[string]int64),
		assetDeltas:   make(map[safetyAssetDeltaKey]int64),
		authChanges:   make(map[string]string),
		appIDsTouched: make(map[uint64]struct{}),
		riskSignals:   make(map[string]struct{}),
	}
}

func accumulateSummaryFromSimulation(acc *safetyAccumulator, response PreEncodedSimulateResponse) {
	for _, group := range response.TxnGroups {
		for _, txn := range group.Txns {
			accumulateSafetyTxn(acc, txn.Txn, false)
		}
	}
}

func newSafetySummary(round basics.Round, acc *safetyAccumulator) SafetyOutcomeSummary {
	return SafetyOutcomeSummary{
		Round:             round,
		AlgoDeltas:        make([]SafetyAlgoDelta, 0, len(acc.algoDeltas)),
		AssetDeltas:       make([]SafetyAssetDelta, 0, len(acc.assetDeltas)),
		AlgoTransfers:     slices.Clone(acc.algoTransfers),
		AssetTransfers:    slices.Clone(acc.assetTransfers),
		AppStateChanges:   slices.Clone(acc.appStateChanges),
		AuthChanges:       make([]SafetyAuthChange, 0, len(acc.authChanges)),
		ResourceLifecycle: slices.Clone(acc.resourceLifecycle),
		AppIDsTouched:     make([]uint64, 0, len(acc.appIDsTouched)),
		InnerTxnCount:     acc.innerTxnCount,
		RiskSignals:       make([]string, 0, len(acc.riskSignals)),
	}
}

func appendSummaryMapValues(summary *SafetyOutcomeSummary, acc *safetyAccumulator) {
	appendAlgoDeltas(summary, acc.algoDeltas)
	appendAssetDeltas(summary, acc.assetDeltas)
	appendAuthChanges(summary, acc.authChanges)
	appendAppIDsTouched(summary, acc.appIDsTouched)
	appendRiskSignals(summary, acc.riskSignals)
}

func appendAlgoDeltas(summary *SafetyOutcomeSummary, deltas map[string]int64) {
	for address, delta := range deltas {
		if delta != 0 {
			summary.AlgoDeltas = append(summary.AlgoDeltas, SafetyAlgoDelta{Address: address, Delta: delta})
		}
	}
}

func appendAssetDeltas(summary *SafetyOutcomeSummary, deltas map[safetyAssetDeltaKey]int64) {
	for key, delta := range deltas {
		if delta != 0 {
			summary.AssetDeltas = append(summary.AssetDeltas, SafetyAssetDelta{Address: key.Address, AssetID: key.AssetID, Delta: delta})
		}
	}
}

func appendAuthChanges(summary *SafetyOutcomeSummary, authChanges map[string]string) {
	for address, rekeyTo := range authChanges {
		summary.AuthChanges = append(summary.AuthChanges, SafetyAuthChange{Address: address, RekeyTo: rekeyTo})
	}
}

func appendAppIDsTouched(summary *SafetyOutcomeSummary, appIDs map[uint64]struct{}) {
	for appID := range appIDs {
		summary.AppIDsTouched = append(summary.AppIDsTouched, appID)
	}
}

func appendRiskSignals(summary *SafetyOutcomeSummary, riskSignals map[string]struct{}) {
	for signal := range riskSignals {
		summary.RiskSignals = append(summary.RiskSignals, signal)
	}
}

func computeSafetyOutcomeHash(response PreEncodedSimulateResponse) string {
	txgroup := buildAttestedTxGroup(response)
	digest := transactions.ComputeGroupOutcomeHash(txgroup)
	return fmt.Sprintf("%x", digest[:])
}

func evaluateSafetyPolicy(summary SafetyOutcomeSummary, policy SafetyPolicy) []SafetyViolation {
	violations := make([]SafetyViolation, 0)
	enforcement := normalizeEnforcementMode(policy.EnforcementMode)
	highRiskSignals := normalizeHighRiskSignals(policy.HighRiskSignals)

	appendAlgoOutflowViolations(&violations, summary, policy)
	appendAssetOutflowViolations(&violations, summary, policy)
	appendRekeyViolations(&violations, summary, policy)
	appendRiskSignalViolations(&violations, summary, policy, enforcement, highRiskSignals)
	appendAppGlobalDeleteViolations(&violations, summary, policy)
	appendInnerTxnViolations(&violations, summary, policy)
	appendReceiverViolations(&violations, summary, policy)
	appendAppIDViolations(&violations, summary, policy)
	sortSafetyViolations(violations)

	return violations
}

func appendAlgoOutflowViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy) {
	if policy.MaxAlgoOutflow == nil {
		return
	}
	var outflow uint64
	for _, delta := range summary.AlgoDeltas {
		if delta.Delta < 0 {
			outflow += uint64(-delta.Delta)
		}
	}
	if outflow > *policy.MaxAlgoOutflow {
		*violations = append(*violations, SafetyViolation{Code: "algo_outflow_exceeded", Path: "algo-deltas", Message: fmt.Sprintf("algo outflow %d exceeds max %d", outflow, *policy.MaxAlgoOutflow), Severity: "error"})
	}
}

func appendAssetOutflowViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy) {
	if policy.MaxAssetOutflow == nil {
		return
	}
	assetOutflow := make(map[uint64]uint64)
	for _, delta := range summary.AssetDeltas {
		if delta.Delta < 0 {
			assetOutflow[delta.AssetID] += uint64(-delta.Delta)
		}
	}
	for assetID, maxOutflow := range policy.MaxAssetOutflow {
		if assetOutflow[assetID] > maxOutflow {
			*violations = append(*violations, SafetyViolation{Code: "asset_outflow_exceeded", Path: fmt.Sprintf("asset-deltas[%d]", assetID), Message: fmt.Sprintf("asset %d outflow %d exceeds max %d", assetID, assetOutflow[assetID], maxOutflow), Severity: "error"})
		}
	}
}

func appendRekeyViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy) {
	if policy.AllowRekey != nil && !*policy.AllowRekey && len(summary.AuthChanges) > 0 {
		*violations = append(*violations, SafetyViolation{Code: "rekey_not_allowed", Path: "auth-changes", Message: "transaction group includes rekey changes", Severity: "error"})
	}
}

func appendRiskSignalViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy, enforcement string, highRiskSignals map[string]struct{}) {
	if len(policy.RequiredRiskSignalsAbsent) == 0 {
		return
	}
	signalSet := makeStringSet(summary.RiskSignals)
	for _, blocked := range policy.RequiredRiskSignalsAbsent {
		if _, present := signalSet[blocked]; present {
			severity := riskSignalSeverity(blocked, enforcement, highRiskSignals)
			*violations = append(*violations, SafetyViolation{Code: "risk_signal_present", Path: "risk-signals", Message: fmt.Sprintf("risk signal %s is present", blocked), Severity: severity})
		}
	}
}

func riskSignalSeverity(signal string, enforcement string, highRiskSignals map[string]struct{}) string {
	if enforcement == SafetyEnforcementHard {
		return "error"
	}
	if _, highRisk := highRiskSignals[signal]; highRisk {
		return "error"
	}
	return "warning"
}

func appendAppGlobalDeleteViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy) {
	if policy.AllowAppGlobalDeletes == nil || *policy.AllowAppGlobalDeletes {
		return
	}
	for _, change := range summary.AppStateChanges {
		if change.Scope == "global" && change.Action == 3 {
			*violations = append(*violations, SafetyViolation{Code: "app_global_delete_not_allowed", Path: "app-state-changes", Message: fmt.Sprintf("global state delete detected for app %d", change.AppID), Severity: "error"})
			return
		}
	}
}

func appendInnerTxnViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy) {
	if policy.MaxInnerTxns != nil && summary.InnerTxnCount > uint64(*policy.MaxInnerTxns) {
		*violations = append(*violations, SafetyViolation{Code: "inner_txn_limit_exceeded", Path: "inner-txn-count", Message: fmt.Sprintf("inner txn count %d exceeds max %d", summary.InnerTxnCount, *policy.MaxInnerTxns), Severity: "error"})
	}
}

func appendReceiverViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy) {
	allowedReceivers := makeStringSet(policy.AllowedReceivers)
	blockedReceivers := makeStringSet(policy.BlockedReceivers)
	for _, transfer := range summary.AlgoTransfers {
		appendReceiverTransferViolations(violations, transfer.To, "algo-transfers", allowedReceivers, blockedReceivers)
	}
	for _, transfer := range summary.AssetTransfers {
		appendReceiverTransferViolations(violations, transfer.To, "asset-transfers", allowedReceivers, blockedReceivers)
	}
}

func appendReceiverTransferViolations(violations *[]SafetyViolation, receiver string, path string, allowed map[string]struct{}, blocked map[string]struct{}) {
	if len(allowed) > 0 {
		if _, ok := allowed[receiver]; !ok {
			*violations = append(*violations, SafetyViolation{Code: "receiver_not_allowed", Path: path, Message: fmt.Sprintf("receiver %s is not in allowed receiver list", receiver), Severity: "error"})
		}
	}
	if _, isBlocked := blocked[receiver]; isBlocked {
		*violations = append(*violations, SafetyViolation{Code: "receiver_blocked", Path: path, Message: fmt.Sprintf("receiver %s is blocked", receiver), Severity: "error"})
	}
}

func appendAppIDViolations(violations *[]SafetyViolation, summary SafetyOutcomeSummary, policy SafetyPolicy) {
	allowedApps := makeUintSet(policy.AllowedAppIDs)
	blockedApps := makeUintSet(policy.BlockedAppIDs)
	for _, appID := range summary.AppIDsTouched {
		if len(allowedApps) > 0 {
			if _, ok := allowedApps[appID]; !ok {
				*violations = append(*violations, SafetyViolation{Code: "app_not_allowed", Path: "app-ids-touched", Message: fmt.Sprintf("app %d is not in allowed app list", appID), Severity: "error"})
			}
		}
		if _, blocked := blockedApps[appID]; blocked {
			*violations = append(*violations, SafetyViolation{Code: "app_blocked", Path: "app-ids-touched", Message: fmt.Sprintf("app %d is blocked", appID), Severity: "error"})
		}
	}
}

func sortSafetyViolations(violations []SafetyViolation) {
	slices.SortFunc(violations, func(a, b SafetyViolation) int {
		if a.Code == b.Code {
			if a.Path == b.Path {
				return strings.Compare(a.Message, b.Message)
			}
			return strings.Compare(a.Path, b.Path)
		}
		return strings.Compare(a.Code, b.Code)
	})
}

func summarizePolicyDecision(policy SafetyPolicy, violations []SafetyViolation) (decision string, hardStop bool, canBypass bool, enforcement string, allowed bool) {
	enforcement = normalizeEnforcementMode(policy.EnforcementMode)
	hasError := false
	for _, violation := range violations {
		if violation.Severity == "error" {
			hasError = true
			break
		}
	}

	if !hasError && len(violations) == 0 {
		return SafetyDecisionAllow, false, true, enforcement, true
	}
	if enforcement == SafetyEnforcementHard {
		return SafetyDecisionBlock, true, false, enforcement, false
	}
	if hasError {
		return SafetyDecisionWarn, true, true, enforcement, true
	}
	return SafetyDecisionWarn, false, true, enforcement, true
}

func unverifiedEvaluateResponse(policy SafetyPolicy, reason string) PreEncodedSafetyEvaluateResponse {
	enforcement := normalizeEnforcementMode(policy.EnforcementMode)
	canBypass := true
	if policy.AllowBypassIfUnavailable != nil {
		canBypass = *policy.AllowBypassIfUnavailable
	}
	allowed := canBypass
	hardStop := !canBypass
	return PreEncodedSafetyEvaluateResponse{
		Allowed:            allowed,
		Violations:         nil,
		Decision:           SafetyDecisionUnverified,
		HardStop:           hardStop,
		CanBypass:          canBypass,
		EnforcementMode:    enforcement,
		VerificationStatus: SafetyVerificationUnavailable,
		UnavailableReason:  reason,
	}
}

func unverifiedSimulateResponse(reason string) PreEncodedSafetySimulateResponse {
	return PreEncodedSafetySimulateResponse{
		VerificationStatus: SafetyVerificationUnavailable,
		UnavailableReason:  reason,
	}
}

func normalizeEnforcementMode(mode string) string {
	if strings.EqualFold(mode, SafetyEnforcementHard) {
		return SafetyEnforcementHard
	}
	return SafetyEnforcementSoft
}

func normalizeHighRiskSignals(input []string) map[string]struct{} {
	if len(input) == 0 {
		input = []string{"rekeyChange", "closeRemainderToUsed", "assetCloseToUsed", "appGlobalStateDelete"}
	}
	set := make(map[string]struct{}, len(input))
	for _, signal := range input {
		if signal == "" {
			continue
		}
		set[signal] = struct{}{}
	}
	return set
}

func deriveRiskLevel(signals []string) string {
	if len(signals) == 0 {
		return "low"
	}
	highRisk := normalizeHighRiskSignals(nil)
	for _, signal := range signals {
		if _, ok := highRisk[signal]; ok {
			return "high"
		}
	}
	return "medium"
}

func buildHumanSummary(summary SafetyOutcomeSummary) []string {
	lines := make([]string, 0, 4)
	if len(summary.AlgoTransfers) > 0 {
		transfer := summary.AlgoTransfers[0]
		lines = append(lines, fmt.Sprintf("Algo transfer: %d microAlgos from %s to %s", transfer.Amount, transfer.From, transfer.To))
	}
	if len(summary.AssetTransfers) > 0 {
		transfer := summary.AssetTransfers[0]
		lines = append(lines, fmt.Sprintf("ASA transfer: asset %d amount %d from %s to %s", transfer.AssetID, transfer.Amount, transfer.From, transfer.To))
	}
	if len(summary.AppIDsTouched) > 0 {
		lines = append(lines, fmt.Sprintf("Applications touched: %d", len(summary.AppIDsTouched)))
	}
	if len(summary.RiskSignals) == 0 {
		lines = append(lines, "No deterministic risk signals detected")
	} else {
		lines = append(lines, fmt.Sprintf("Risk signals: %s", strings.Join(summary.RiskSignals, ",")))
	}
	return lines
}

func makeStringSet(values []string) map[string]struct{} {
	if len(values) == 0 {
		return nil
	}
	set := make(map[string]struct{}, len(values))
	for _, value := range values {
		if value == "" {
			continue
		}
		set[value] = struct{}{}
	}
	return set
}

func makeUintSet(values []uint64) map[uint64]struct{} {
	if len(values) == 0 {
		return nil
	}
	set := make(map[uint64]struct{}, len(values))
	for _, value := range values {
		set[value] = struct{}{}
	}
	return set
}

func buildAttestedTxGroup(response PreEncodedSimulateResponse) []transactions.SignedTxnWithAD {
	if len(response.TxnGroups) == 0 {
		return nil
	}
	group := make([]transactions.SignedTxnWithAD, 0, len(response.TxnGroups[0].Txns))
	for _, txn := range response.TxnGroups[0].Txns {
		group = append(group, buildAttestedTxn(txn.Txn))
	}
	return group
}

func buildAttestedTxn(info PreEncodedTxInfo) transactions.SignedTxnWithAD {
	result := transactions.SignedTxnWithAD{SignedTxn: info.Txn}
	if info.ClosingAmount != nil {
		result.ApplyData.ClosingAmount.Raw = *info.ClosingAmount
	}
	if info.AssetClosingAmount != nil {
		result.ApplyData.AssetClosingAmount = *info.AssetClosingAmount
	}
	if info.AssetIndex != nil {
		result.ApplyData.ConfigAsset = *info.AssetIndex
	}
	if info.ApplicationIndex != nil {
		result.ApplyData.ApplicationID = *info.ApplicationIndex
	}
	if info.Inners != nil {
		result.ApplyData.EvalDelta.InnerTxns = make([]transactions.SignedTxnWithAD, 0, len(*info.Inners))
		for _, inner := range *info.Inners {
			result.ApplyData.EvalDelta.InnerTxns = append(result.ApplyData.EvalDelta.InnerTxns, buildAttestedTxn(inner))
		}
	}
	return result
}

func sortSafetySummary(summary *SafetyOutcomeSummary) {
	slices.SortFunc(summary.AlgoDeltas, compareAlgoDelta)
	slices.SortFunc(summary.AssetDeltas, compareAssetDelta)
	slices.SortFunc(summary.AlgoTransfers, compareAlgoTransfer)
	slices.SortFunc(summary.AssetTransfers, compareAssetTransfer)
	slices.SortFunc(summary.AppStateChanges, compareAppStateChange)
	slices.SortFunc(summary.AuthChanges, compareAuthChange)
	slices.SortFunc(summary.ResourceLifecycle, compareResourceLifecycle)
	slices.Sort(summary.AppIDsTouched)
	slices.Sort(summary.RiskSignals)
}

func compareAlgoDelta(a, b SafetyAlgoDelta) int {
	if a.Address != b.Address {
		return strings.Compare(a.Address, b.Address)
	}
	if a.Delta < b.Delta {
		return -1
	}
	if a.Delta > b.Delta {
		return 1
	}
	return 0
}

func compareAssetDelta(a, b SafetyAssetDelta) int {
	if a.AssetID != b.AssetID {
		if a.AssetID < b.AssetID {
			return -1
		}
		return 1
	}
	if a.Address != b.Address {
		return strings.Compare(a.Address, b.Address)
	}
	if a.Delta < b.Delta {
		return -1
	}
	if a.Delta > b.Delta {
		return 1
	}
	return 0
}

func compareAlgoTransfer(a, b SafetyAlgoTransfer) int {
	if a.From != b.From {
		return strings.Compare(a.From, b.From)
	}
	if a.To != b.To {
		return strings.Compare(a.To, b.To)
	}
	if a.Amount != b.Amount {
		if a.Amount < b.Amount {
			return -1
		}
		return 1
	}
	return strings.Compare(a.Kind, b.Kind)
}

func compareAssetTransfer(a, b SafetyAssetTransfer) int {
	if a.AssetID != b.AssetID {
		if a.AssetID < b.AssetID {
			return -1
		}
		return 1
	}
	if a.From != b.From {
		return strings.Compare(a.From, b.From)
	}
	if a.To != b.To {
		return strings.Compare(a.To, b.To)
	}
	if a.Amount != b.Amount {
		if a.Amount < b.Amount {
			return -1
		}
		return 1
	}
	return strings.Compare(a.Kind, b.Kind)
}

func compareAppStateChange(a, b SafetyAppStateChange) int {
	if a.AppID != b.AppID {
		if a.AppID < b.AppID {
			return -1
		}
		return 1
	}
	if a.Scope != b.Scope {
		return strings.Compare(a.Scope, b.Scope)
	}
	if a.Account != b.Account {
		return strings.Compare(a.Account, b.Account)
	}
	if a.KeyB64 != b.KeyB64 {
		return strings.Compare(a.KeyB64, b.KeyB64)
	}
	if a.Action < b.Action {
		return -1
	}
	if a.Action > b.Action {
		return 1
	}
	return 0
}

func compareAuthChange(a, b SafetyAuthChange) int {
	if a.Address != b.Address {
		return strings.Compare(a.Address, b.Address)
	}
	return strings.Compare(a.RekeyTo, b.RekeyTo)
}

func compareResourceLifecycle(a, b SafetyResourceLifecycle) int {
	if a.Kind != b.Kind {
		return strings.Compare(a.Kind, b.Kind)
	}
	if a.Action != b.Action {
		return strings.Compare(a.Action, b.Action)
	}
	if a.ID != b.ID {
		if a.ID < b.ID {
			return -1
		}
		return 1
	}
	return strings.Compare(a.Address, b.Address)
}

func accumulateSafetyTxn(acc *safetyAccumulator, info PreEncodedTxInfo, isInner bool) {
	txn := info.Txn.Txn
	sender := txn.Sender.String()
	addAlgoDelta(acc.algoDeltas, sender, -int64(txn.Fee.Raw))
	if !txn.RekeyTo.IsZero() {
		acc.authChanges[sender] = txn.RekeyTo.String()
		acc.riskSignals["rekeyChange"] = struct{}{}
	}
	switch txn.Type {
	case protocol.PaymentTx:
		handleSafetyPayment(acc, info)
	case protocol.AssetTransferTx:
		handleSafetyAssetTransfer(acc, info)
	case protocol.AssetConfigTx:
		handleSafetyAssetConfig(acc, info)
	case protocol.ApplicationCallTx:
		handleSafetyAppCall(acc, info)
	}
	collectSafetyAppStateChanges(acc, info)
	if isInner {
		acc.innerTxnCount++
	}
	if info.Inners != nil {
		for _, inner := range *info.Inners {
			accumulateSafetyTxn(acc, inner, true)
		}
	}
}

func handleSafetyPayment(acc *safetyAccumulator, info PreEncodedTxInfo) {
	txn := info.Txn.Txn
	from := txn.Sender.String()
	to := txn.Receiver.String()
	if txn.Amount.Raw > 0 {
		addAlgoDelta(acc.algoDeltas, from, -int64(txn.Amount.Raw))
		addAlgoDelta(acc.algoDeltas, to, int64(txn.Amount.Raw))
		acc.algoTransfers = append(acc.algoTransfers, SafetyAlgoTransfer{From: from, To: to, Amount: txn.Amount.Raw, Kind: "pay"})
	}
	if !txn.CloseRemainderTo.IsZero() && info.ClosingAmount != nil && *info.ClosingAmount > 0 {
		closeTo := txn.CloseRemainderTo.String()
		addAlgoDelta(acc.algoDeltas, from, -int64(*info.ClosingAmount))
		addAlgoDelta(acc.algoDeltas, closeTo, int64(*info.ClosingAmount))
		acc.algoTransfers = append(acc.algoTransfers, SafetyAlgoTransfer{From: from, To: closeTo, Amount: *info.ClosingAmount, Kind: "close"})
		acc.riskSignals["closeRemainderToUsed"] = struct{}{}
	}
}

func handleSafetyAssetTransfer(acc *safetyAccumulator, info PreEncodedTxInfo) {
	txn := info.Txn.Txn
	assetID := uint64(txn.XferAsset)
	if assetID == 0 {
		return
	}
	fromAddr := txn.Sender
	if !txn.AssetSender.IsZero() {
		fromAddr = txn.AssetSender
	}
	from := fromAddr.String()
	to := txn.AssetReceiver.String()
	if txn.AssetAmount > 0 {
		addAssetDelta(acc.assetDeltas, from, assetID, -int64(txn.AssetAmount))
		addAssetDelta(acc.assetDeltas, to, assetID, int64(txn.AssetAmount))
		acc.assetTransfers = append(acc.assetTransfers, SafetyAssetTransfer{AssetID: assetID, From: from, To: to, Amount: txn.AssetAmount, Kind: "axfer"})
	}
	if !txn.AssetCloseTo.IsZero() && info.AssetClosingAmount != nil && *info.AssetClosingAmount > 0 {
		closeTo := txn.AssetCloseTo.String()
		addAssetDelta(acc.assetDeltas, from, assetID, -int64(*info.AssetClosingAmount))
		addAssetDelta(acc.assetDeltas, closeTo, assetID, int64(*info.AssetClosingAmount))
		acc.assetTransfers = append(acc.assetTransfers, SafetyAssetTransfer{AssetID: assetID, From: from, To: closeTo, Amount: *info.AssetClosingAmount, Kind: "aclose"})
		acc.riskSignals["assetCloseToUsed"] = struct{}{}
	}
}

func handleSafetyAssetConfig(acc *safetyAccumulator, info PreEncodedTxInfo) {
	txn := info.Txn.Txn
	creator := txn.Sender.String()
	if txn.ConfigAsset == 0 {
		createdID := uint64(0)
		if info.AssetIndex != nil {
			createdID = uint64(*info.AssetIndex)
		}
		acc.resourceLifecycle = append(acc.resourceLifecycle, SafetyResourceLifecycle{Kind: "asset", Action: "create", ID: createdID, Address: creator})
		return
	}
	if txn.AssetParams == (basics.AssetParams{}) {
		acc.resourceLifecycle = append(acc.resourceLifecycle, SafetyResourceLifecycle{Kind: "asset", Action: "delete", ID: uint64(txn.ConfigAsset), Address: creator})
		return
	}
	acc.resourceLifecycle = append(acc.resourceLifecycle, SafetyResourceLifecycle{Kind: "asset", Action: "reconfigure", ID: uint64(txn.ConfigAsset), Address: creator})
}

func handleSafetyAppCall(acc *safetyAccumulator, info PreEncodedTxInfo) {
	txn := info.Txn.Txn
	appID := uint64(txn.ApplicationID)
	if appID == 0 && info.ApplicationIndex != nil {
		appID = uint64(*info.ApplicationIndex)
	}
	if appID > 0 {
		acc.appIDsTouched[appID] = struct{}{}
	}
	sender := txn.Sender.String()
	if txn.ApplicationID == 0 {
		createdID := uint64(0)
		if info.ApplicationIndex != nil {
			createdID = uint64(*info.ApplicationIndex)
		}
		acc.resourceLifecycle = append(acc.resourceLifecycle, SafetyResourceLifecycle{Kind: "app", Action: "create", ID: createdID, Address: sender})
		return
	}
	if txn.OnCompletion == transactions.DeleteApplicationOC {
		acc.resourceLifecycle = append(acc.resourceLifecycle, SafetyResourceLifecycle{Kind: "app", Action: "delete", ID: uint64(txn.ApplicationID), Address: sender})
		acc.riskSignals["appGlobalStateDelete"] = struct{}{}
	}
}

func collectSafetyAppStateChanges(acc *safetyAccumulator, info PreEncodedTxInfo) {
	txn := info.Txn.Txn
	appID := uint64(txn.ApplicationID)
	if appID == 0 && info.ApplicationIndex != nil {
		appID = uint64(*info.ApplicationIndex)
	}
	if info.GlobalStateDelta != nil {
		for _, kv := range *info.GlobalStateDelta {
			acc.appStateChanges = append(acc.appStateChanges, SafetyAppStateChange{AppID: appID, Scope: "global", KeyB64: kv.Key, Action: kv.Value.Action})
			if kv.Value.Action == 3 {
				acc.riskSignals["appGlobalStateDelete"] = struct{}{}
			}
		}
	}
	if info.LocalStateDelta != nil {
		for _, local := range *info.LocalStateDelta {
			for _, kv := range local.Delta {
				acc.appStateChanges = append(acc.appStateChanges, SafetyAppStateChange{AppID: appID, Scope: "local", Account: local.Address, KeyB64: kv.Key, Action: kv.Value.Action})
			}
		}
	}
}

func addAlgoDelta(deltas map[string]int64, addr string, delta int64) {
	if addr == "" || delta == 0 {
		return
	}
	deltas[addr] += delta
}

func addAssetDelta(deltas map[safetyAssetDeltaKey]int64, addr string, assetID uint64, delta int64) {
	if addr == "" || assetID == 0 || delta == 0 {
		return
	}
	key := safetyAssetDeltaKey{Address: addr, AssetID: assetID}
	deltas[key] += delta
}
