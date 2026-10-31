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
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
	"github.com/algorand/go-algorand/ledger/apply"
)

func (eval *BlockEvaluator) applyClosedAssetDestruction(tx transactions.Transaction, cs *roundCowState, ad *transactions.ApplyData, counter uint64, marker retirement.Record, record closure.Record) error {
	child := cs.child(0)
	defer child.recycle()
	if err := child.checkAssetPrivacyClosingDestruction(tx, marker, record); err != nil {
		return err
	}
	if err := child.removeAssetPrivacyClosingRecords(tx.ConfigAsset); err != nil {
		return err
	}
	if err := eval.applyClosingAssetConfig(tx, child, ad, counter); err != nil {
		return err
	}
	if err := child.requireAssetPrivacyAbsent(tx.ConfigAsset); err != nil {
		return err
	}
	child.commitToParent()
	return nil
}

func (cs *roundCowState) checkAssetPrivacyClosingDestruction(tx transactions.Transaction, marker retirement.Record, record closure.Record) error {
	if record.Destroy != [32]byte(tx.ID()) || record.Manager != tx.Sender ||
		record.Created != uint64(cs.Round()) || record.Epoch != 1 || record.Policy != marker.Policy || record.Suite != marker.Suite {
		return closure.ErrRecord
	}
	if err := cs.checkAssetPrivacyDestructionContext(tx.ConfigAsset, tx.Sender, marker); err != nil {
		return err
	}
	return cs.checkAssetPrivacyClosureParameters(tx.ConfigAsset, record)
}

func (eval *BlockEvaluator) applyClosingAssetConfig(tx transactions.Transaction, child *roundCowState, ad *transactions.ApplyData, counter uint64) error {
	child.nativePrivacyDestructionAsset = tx.ConfigAsset
	defer func() { child.nativePrivacyDestructionAsset = 0 }()
	return apply.AssetConfig(tx.AssetConfigTxnFields, tx.Header, child, eval.specials, ad, counter)
}
