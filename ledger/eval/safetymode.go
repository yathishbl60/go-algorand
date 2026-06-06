package eval

import (
	"fmt"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/data/transactions"
)

func validateGroupSafetyAttestation(txgroup []transactions.SignedTxnWithAD, txibs []transactions.SignedTxnInBlock) error {
	mode, expectedHash, err := resolveGroupSafetyMode(txgroup)
	if err != nil {
		return err
	}
	if mode == transactions.SafetyModeDisabled {
		return nil
	}
	if expectedHash.IsZero() {
		return fmt.Errorf("safety mode enabled but expected outcome hash is empty")
	}

	completed, err := buildCompletedAttestedGroup(txgroup, txibs, mode, expectedHash)
	if err != nil {
		return err
	}
	if err = verifyOutcomeHash(completed, expectedHash); err != nil {
		return err
	}
	return verifyRiskSignals(mode, completed)
}

func resolveGroupSafetyMode(txgroup []transactions.SignedTxnWithAD) (uint8, crypto.Digest, error) {
	mode := transactions.SafetyModeDisabled
	expectedHash := crypto.Digest{}
	for _, txad := range txgroup {
		tx := txad.SignedTxn.Txn
		if tx.SafetyMode > transactions.SafetyModeHashAndRiskEnforced {
			return mode, expectedHash, fmt.Errorf("invalid safety mode %d", tx.SafetyMode)
		}
		if tx.SafetyMode == transactions.SafetyModeDisabled {
			continue
		}
		if mode == transactions.SafetyModeDisabled {
			mode = tx.SafetyMode
			expectedHash = tx.ExpectedOutcomeHash
			continue
		}
		if tx.SafetyMode != mode {
			return mode, expectedHash, fmt.Errorf("inconsistent safety mode in transaction group")
		}
		if tx.ExpectedOutcomeHash != expectedHash {
			return mode, expectedHash, fmt.Errorf("inconsistent expected outcome hash in transaction group")
		}
	}
	return mode, expectedHash, nil
}

func buildCompletedAttestedGroup(txgroup []transactions.SignedTxnWithAD, txibs []transactions.SignedTxnInBlock, mode uint8, expectedHash crypto.Digest) ([]transactions.SignedTxnWithAD, error) {
	completed := make([]transactions.SignedTxnWithAD, len(txgroup))
	for i := range txgroup {
		tx := txgroup[i].SignedTxn.Txn
		if tx.SafetyMode != mode || tx.ExpectedOutcomeHash != expectedHash {
			return nil, fmt.Errorf("all transactions in attested group must use identical safety mode and expected outcome hash")
		}
		completed[i] = transactions.SignedTxnWithAD{SignedTxn: txgroup[i].SignedTxn, ApplyData: txibs[i].ApplyData}
	}
	return completed, nil
}

func verifyOutcomeHash(completed []transactions.SignedTxnWithAD, expectedHash crypto.Digest) error {
	if transactions.ComputeGroupOutcomeHash(completed) != expectedHash {
		return fmt.Errorf("safety outcome hash mismatch")
	}
	return nil
}

func verifyRiskSignals(mode uint8, completed []transactions.SignedTxnWithAD) error {
	if mode != transactions.SafetyModeHashAndRiskEnforced {
		return nil
	}
	riskSignals := transactions.CollectGroupRiskSignals(completed)
	if len(riskSignals) > 0 {
		return fmt.Errorf("safety mode rejected transaction group due to risk signals: %v", riskSignals)
	}
	return nil
}
