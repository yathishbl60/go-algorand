package v2

import (
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestEvaluateSafetyPolicyCoversAllRules(t *testing.T) {
	t.Parallel()
	violations := evaluateSafetyPolicy(buildComprehensiveSummary(), buildComprehensivePolicy())
	assertComprehensiveViolationCodes(t, violations)
}

func buildComprehensiveSummary() SafetyOutcomeSummary {
	return SafetyOutcomeSummary{
		AlgoDeltas:      []SafetyAlgoDelta{{Address: "A", Delta: -100}},
		AssetDeltas:     []SafetyAssetDelta{{Address: "A", AssetID: 7, Delta: -200}},
		AuthChanges:     []SafetyAuthChange{{Address: "A", RekeyTo: "B"}},
		RiskSignals:     []string{"rekeyChange", "customSignal"},
		AppStateChanges: []SafetyAppStateChange{{AppID: 9, Scope: "global", Action: 3}},
		InnerTxnCount:   3,
		AlgoTransfers:   []SafetyAlgoTransfer{{To: "blocked"}},
		AssetTransfers:  []SafetyAssetTransfer{{To: "other"}},
		AppIDsTouched:   []uint64{42},
	}
}

func buildComprehensivePolicy() SafetyPolicy {
	deny := false
	maxAlgo := uint64(50)
	maxInner := uint32(1)
	return SafetyPolicy{
		MaxAlgoOutflow:            &maxAlgo,
		MaxAssetOutflow:           map[uint64]uint64{7: 100},
		AllowRekey:                &deny,
		RequiredRiskSignalsAbsent: []string{"rekeyChange", "customSignal"},
		AllowAppGlobalDeletes:     &deny,
		MaxInnerTxns:              &maxInner,
		AllowedReceivers:          []string{"allowed-only"},
		BlockedReceivers:          []string{"blocked"},
		AllowedAppIDs:             []uint64{7},
		BlockedAppIDs:             []uint64{42},
		HighRiskSignals:           []string{"customSignal"},
	}
}

func assertComprehensiveViolationCodes(t *testing.T, violations []SafetyViolation) {
	t.Helper()
	requireViolationCode(t, violations, "algo_outflow_exceeded")
	requireViolationCode(t, violations, "asset_outflow_exceeded")
	requireViolationCode(t, violations, "rekey_not_allowed")
	requireViolationCode(t, violations, "risk_signal_present")
	requireViolationCode(t, violations, "app_global_delete_not_allowed")
	requireViolationCode(t, violations, "inner_txn_limit_exceeded")
	requireViolationCode(t, violations, "receiver_not_allowed")
	requireViolationCode(t, violations, "receiver_blocked")
	requireViolationCode(t, violations, "app_not_allowed")
	requireViolationCode(t, violations, "app_blocked")
}

func TestRiskSignalSeverityModes(t *testing.T) {
	t.Parallel()
	summary := SafetyOutcomeSummary{RiskSignals: []string{"r1"}}
	soft := SafetyPolicy{RequiredRiskSignalsAbsent: []string{"r1"}}
	hard := SafetyPolicy{RequiredRiskSignalsAbsent: []string{"r1"}, EnforcementMode: SafetyEnforcementHard}
	high := SafetyPolicy{RequiredRiskSignalsAbsent: []string{"r1"}, HighRiskSignals: []string{"r1"}}

	softViolations := evaluateSafetyPolicy(summary, soft)
	require.Equal(t, "warning", softViolations[0].Severity)
	hardViolations := evaluateSafetyPolicy(summary, hard)
	require.Equal(t, "error", hardViolations[0].Severity)
	highViolations := evaluateSafetyPolicy(summary, high)
	require.Equal(t, "error", highViolations[0].Severity)
}

func TestSummarizePolicyDecisionBranches(t *testing.T) {
	t.Parallel()
	decision, hardStop, canBypass, _, allowed := summarizePolicyDecision(SafetyPolicy{}, nil)
	require.Equal(t, SafetyDecisionAllow, decision)
	require.False(t, hardStop)
	require.True(t, canBypass)
	require.True(t, allowed)

	softErr := []SafetyViolation{{Severity: "error"}}
	decision, hardStop, canBypass, _, allowed = summarizePolicyDecision(SafetyPolicy{}, softErr)
	require.Equal(t, SafetyDecisionWarn, decision)
	require.True(t, hardStop)
	require.True(t, canBypass)
	require.True(t, allowed)

	hardPolicy := SafetyPolicy{EnforcementMode: SafetyEnforcementHard}
	decision, hardStop, canBypass, _, allowed = summarizePolicyDecision(hardPolicy, softErr)
	require.Equal(t, SafetyDecisionBlock, decision)
	require.True(t, hardStop)
	require.False(t, canBypass)
	require.False(t, allowed)

	warnOnly := []SafetyViolation{{Severity: "warning"}}
	decision, hardStop, canBypass, _, allowed = summarizePolicyDecision(hardPolicy, warnOnly)
	require.Equal(t, SafetyDecisionBlock, decision)
	require.True(t, hardStop)
	require.False(t, canBypass)
	require.False(t, allowed)
}

func TestUnverifiedEvaluateResponseBypassPolicy(t *testing.T) {
	t.Parallel()
	resp := unverifiedEvaluateResponse(SafetyPolicy{}, "down")
	require.Equal(t, SafetyDecisionUnverified, resp.Decision)
	require.True(t, resp.CanBypass)
	require.False(t, resp.HardStop)
	require.True(t, resp.Allowed)

	deny := false
	resp = unverifiedEvaluateResponse(SafetyPolicy{AllowBypassIfUnavailable: &deny}, "down")
	require.False(t, resp.CanBypass)
	require.True(t, resp.HardStop)
	require.False(t, resp.Allowed)
}

func TestSafetyCacheTTLAndLRU(t *testing.T) {
	t.Parallel()
	cache := newSafetyCache(2, 20*time.Millisecond)
	cache.set("k1", 1)
	cache.set("k2", 2)
	cache.set("k3", 3)

	_, ok := cache.get("k1")
	require.False(t, ok)
	v, ok := cache.get("k3")
	require.True(t, ok)
	require.Equal(t, 3, v)

	cache.set("ttl", "x")
	time.Sleep(30 * time.Millisecond)
	_, ok = cache.get("ttl")
	require.False(t, ok)
}

func requireViolationCode(t *testing.T, violations []SafetyViolation, code string) {
	t.Helper()
	for _, violation := range violations {
		if violation.Code == code {
			return
		}
	}
	require.Fail(t, "missing violation code", code)
}
