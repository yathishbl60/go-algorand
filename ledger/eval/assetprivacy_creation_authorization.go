//go:build algorand_native_privacy

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
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/protocol"
)

func (eval *BlockEvaluator) checkAssetPrivacyTransactionShape(tx transactions.SignedTxn, cs *roundCowState) error {
	if !cs.proto.EnableNativeAssetPrivacy {
		if tx.Txn.Type == protocol.NativeAssetPrivacyTx || len(tx.Txn.NativeAssetPrivacy) != 0 || len(tx.NativeAssetPrivacyProof) != 0 {
			return policy.ErrDisabled
		}
		return nil
	}
	if tx.Txn.IsNativeAssetPrivacyCreation() {
		return eval.checkAssetPrivacyCreationSigned(tx, cs)
	}
	if tx.Txn.Type == protocol.NativeAssetPrivacyTx {
		return cs.checkAssetPrivacyExecutionShape(tx)
	}
	if len(tx.Txn.NativeAssetPrivacy) != 0 || len(tx.NativeAssetPrivacyProof) != 0 {
		return errAssetPrivacyAuthorization
	}
	return nil
}

func (eval *BlockEvaluator) checkAssetPrivacyCreationSigned(tx transactions.SignedTxn, cs *roundCowState) error {
	if len(tx.NativeAssetPrivacyProof) != 0 {
		return errAssetPrivacyAuthorization
	}
	if err := cs.checkAssetPrivacyExecutionShape(tx); err != nil {
		return err
	}
	if _, err := cs.assetPrivacyCreationBody(tx.Txn); err != nil {
		return err
	}
	return tx.Txn.WellFormed(eval.specials, cs.proto)
}
