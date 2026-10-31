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
	"github.com/algorand/go-algorand/data/transactions/logic"
	"github.com/algorand/go-algorand/ledger/apply"
)

func (eval *BlockEvaluator) applyAssetConfigWithPrivacy(tx transactions.Transaction, cs *roundCowState, params *logic.EvalParams, gi int, ad *transactions.ApplyData, counter uint64) error {
	if !tx.IsNativeAssetPrivacyCreation() {
		return eval.applyExistingAssetConfigWithPrivacy(tx, cs, ad, counter)
	}
	body, err := eval.assetPrivacyCreationContext(tx, cs, params, gi)
	if err != nil {
		return err
	}
	child := cs.child(0)
	defer child.recycle()
	if err = apply.AssetConfig(tx.AssetConfigTxnFields, tx.Header, child, eval.specials, ad, counter); err != nil {
		return err
	}
	if err = child.initializeAssetPrivacyCreation(tx, body, ad.ConfigAsset); err != nil {
		return err
	}
	child.commitToParent()
	return nil
}
