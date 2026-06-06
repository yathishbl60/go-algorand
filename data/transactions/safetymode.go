package transactions

import (
	"slices"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/protocol"
)

const (
	// SafetyModeDisabled disables group outcome attestation checks.
	SafetyModeDisabled uint8 = 0
	// SafetyModeHashEnforced requires outcome hash equality.
	SafetyModeHashEnforced uint8 = 1
	// SafetyModeHashAndRiskEnforced requires outcome hash equality and no risk signals.
	SafetyModeHashAndRiskEnforced uint8 = 2
)

const safetyOutcomeHashDomain = "GTSM-OUTCOME-v1"

type safetyHashTxn struct {
	_struct            struct{}        `codec:",omitempty,omitemptyarray"`
	SignedTxn          SignedTxn       `codec:"stxn"`
	ClosingAmount      uint64          `codec:"ca"`
	AssetClosingAmount uint64          `codec:"aca"`
	ConfigAsset        uint64          `codec:"caid"`
	ApplicationID      uint64          `codec:"apid"`
	Inner              []safetyHashTxn `codec:"itx,allocbound=bounds.MaxInnerTransactionsPerDelta"`
}

type safetyHashEnvelope struct {
	_struct struct{}        `codec:",omitempty,omitemptyarray"`
	Txns    []safetyHashTxn `codec:"txns,allocbound=bounds.MaxTxGroupSize"`
}

// ComputeGroupOutcomeHash computes the canonical outcome hash used by safety mode.
func ComputeGroupOutcomeHash(txgroup []SignedTxnWithAD) crypto.Digest {
	env := safetyHashEnvelope{Txns: make([]safetyHashTxn, 0, len(txgroup))}
	for _, txad := range txgroup {
		env.Txns = append(env.Txns, toSafetyHashTxn(txad))
	}
	encoded := protocol.EncodeReflect(&env)
	buf := make([]byte, 0, len(safetyOutcomeHashDomain)+len(encoded))
	buf = append(buf, []byte(safetyOutcomeHashDomain)...)
	buf = append(buf, encoded...)
	return crypto.Hash(buf)
}

// CollectGroupRiskSignals returns deterministic risk signals derived from the group effects.
func CollectGroupRiskSignals(txgroup []SignedTxnWithAD) []string {
	signals := map[string]struct{}{}
	for _, txad := range txgroup {
		collectTxnRiskSignals(txad, signals)
	}
	out := make([]string, 0, len(signals))
	for signal := range signals {
		out = append(out, signal)
	}
	slices.Sort(out)
	return out
}

func toSafetyHashTxn(txad SignedTxnWithAD) safetyHashTxn {
	inner := make([]safetyHashTxn, 0, len(txad.ApplyData.EvalDelta.InnerTxns))
	for _, in := range txad.ApplyData.EvalDelta.InnerTxns {
		inner = append(inner, toSafetyHashTxn(in))
	}
	signedTxn := txad.SignedTxn
	signedTxn.Txn.ExpectedOutcomeHash = crypto.Digest{}
	signedTxn.Txn.SafetyMode = SafetyModeDisabled
	return safetyHashTxn{
		SignedTxn:          signedTxn,
		ClosingAmount:      txad.ApplyData.ClosingAmount.Raw,
		AssetClosingAmount: txad.ApplyData.AssetClosingAmount,
		ConfigAsset:        uint64(txad.ApplyData.ConfigAsset),
		ApplicationID:      uint64(txad.ApplyData.ApplicationID),
		Inner:              inner,
	}
}

func collectTxnRiskSignals(txad SignedTxnWithAD, signals map[string]struct{}) {
	tx := txad.SignedTxn.Txn
	if !tx.RekeyTo.IsZero() {
		signals["rekeyChange"] = struct{}{}
	}
	if !tx.CloseRemainderTo.IsZero() && txad.ApplyData.ClosingAmount.Raw > 0 {
		signals["closeRemainderToUsed"] = struct{}{}
	}
	if !tx.AssetCloseTo.IsZero() && txad.ApplyData.AssetClosingAmount > 0 {
		signals["assetCloseToUsed"] = struct{}{}
	}
	if tx.OnCompletion == DeleteApplicationOC {
		signals["appGlobalStateDelete"] = struct{}{}
	}
	for _, in := range txad.ApplyData.EvalDelta.InnerTxns {
		collectTxnRiskSignals(in, signals)
	}
}
