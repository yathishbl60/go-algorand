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
	"errors"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

var errAssetPrivacyAuthorization = errors.New("native privacy requires current top-level PQ authorization")

func (eval *BlockEvaluator) checkAssetPrivacyAuthorization(tx transactions.SignedTxn, cs *roundCowState) error {
	if err := eval.checkAssetPrivacyTransactionShape(tx, cs); err != nil {
		return err
	}
	if !cs.proto.EnableNativeAssetPrivacy {
		return nil
	}
	required, err := cs.assetPrivacyNeedsPQ(tx.Txn)
	if err != nil || !required {
		return err
	}
	return cs.verifyAssetPrivacyPQ(tx)
}

func (cs *roundCowState) verifyAssetPrivacyPQ(tx transactions.SignedTxn) error {
	if tx.Txn.GenesisHash != cs.GenesisHash() || !tx.Sig.Blank() || !tx.Msig.Blank() || !tx.Lsig.Blank() || len(tx.PQsig.Signature) == 0 {
		return errAssetPrivacyAuthorization
	}
	account, err := cs.Get(tx.Txn.Sender, false)
	if err != nil {
		return err
	}
	authorizer := account.AuthAddr
	if authorizer == (basics.Address{}) {
		authorizer = tx.Txn.Sender
	}
	if tx.Authorizer() != authorizer || !authorizer.IsPQCompliant() {
		return errAssetPrivacyAuthorization
	}
	return tx.PQsig.Verify(cs.proto, tx.Txn, authorizer)
}

func (cs *roundCowState) checkAssetPrivacyExecutionShape(tx transactions.SignedTxn) error {
	if !cs.proto.NativeAssetPrivacyExecutionConfigured() {
		return policy.ErrDisabled
	}
	if !tx.NativeAssetPrivacySizeAllowed(cs.proto) {
		return errAssetPrivacyAuthorization
	}
	return nil
}
