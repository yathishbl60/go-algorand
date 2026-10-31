//go:build !algorand_native_privacy

// Copyright (C) 2019-2026 Algorand Foundation Ltd.
// This file is part of go-algorand
//
// go-algorand is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// go-algorand is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with go-algorand.  If not, see <https://www.gnu.org/licenses/>.

package eval

import (
	"errors"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/data/transactions/logic"
	"github.com/algorand/go-algorand/protocol"
)

func (cs *roundCowState) chargeAssetPrivacyStorage(string, []byte, bool) error { return nil }
func (cs *roundCowState) commitAssetPrivacyProofChecks()                       {}
func (cs *roundCowState) checkAssetPrivacyStorageBalance(basics.Address) error { return nil }
func (cs *roundCowState) assetPrivacyMinimumBalance(_ basics.Address, base basics.MicroAlgos) (basics.MicroAlgos, error) {
	return base, nil
}
func (eval *BlockEvaluator) checkAssetPrivacyAuthorization(tx transactions.SignedTxn, _ *roundCowState) error {
	if tx.Txn.Type == protocol.NativeAssetPrivacyTx || len(tx.Txn.NativeAssetPrivacy) != 0 || len(tx.NativeAssetPrivacyProof) != 0 {
		return errors.New("native asset privacy requires the enabled build")
	}
	return nil
}
func (cs *roundCowState) checkAssetPrivacyInnerAuthorization(tx transactions.Transaction) error {
	if tx.Type == protocol.NativeAssetPrivacyTx || len(tx.NativeAssetPrivacy) != 0 {
		return errors.New("native asset privacy requires the enabled build")
	}
	return nil
}
func (eval *BlockEvaluator) applyNativeAssetPrivacy(*roundCowState, *logic.EvalParams, int) error {
	return errors.New("native asset privacy requires the enabled build")
}
