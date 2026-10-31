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
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

// Authority activation is checked at the atomic group's commit boundary.
func (eval *BlockEvaluator) checkAssetPrivacyAuthorityGroup(cs *roundCowState, group []transactions.SignedTxnWithAD) error {
	if !cs.proto.EnableNativeAssetPrivacy {
		return nil
	}
	for _, signed := range group {
		tx := signed.Txn
		if tx.Type != protocol.AssetConfigTx || tx.ConfigAsset == 0 || tx.AssetParams == (basics.AssetParams{}) {
			continue
		}
		if err := eval.checkAssetPrivacyAuthorityChange(cs, tx.ConfigAsset); err != nil {
			return err
		}
	}
	return nil
}

func (eval *BlockEvaluator) checkAssetPrivacyAuthorityChange(cs *roundCowState, asset basics.AssetIndex) error {
	declaration, exists, err := eval.state.privacyHoldingPolicy(asset)
	if err != nil || !exists {
		return err
	}
	if _, exists, err = cs.GetCreator(basics.CreatableIndex(asset), basics.AssetCreatable); err != nil || !exists {
		return err
	}
	before, _, err := eval.state.AssetParams(asset)
	if err != nil {
		return err
	}
	after, _, err := cs.AssetParams(asset)
	if err != nil {
		return err
	}
	return cs.requireAssetPrivacyAuthorityKeys(asset, before, after, declaration)
}
