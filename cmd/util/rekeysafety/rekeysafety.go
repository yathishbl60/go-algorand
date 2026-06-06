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

package rekeysafety

import (
	"fmt"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

// RekeyTxnInfo describes a transaction in a set that changes account authority.
type RekeyTxnInfo struct {
	Index        int
	Sender       basics.Address
	RekeyTo      basics.Address
	Type         protocol.TxType
	Group        crypto.Digest
	AssetOptIn   bool
	SameGroupKey string
}

// ScanReport summarizes rekey-related risk indicators in a set of transactions.
type ScanReport struct {
	Rekeys                []RekeyTxnInfo
	HasAssetOptInAndRekey bool
}

// Scan reports all rekey transactions and highlights the high-risk pattern of
// ASA opt-in grouped with rekeying.
func Scan(stxns []transactions.SignedTxn) ScanReport {
	report := ScanReport{}
	groupHasRekey := make(map[string]bool)
	groupHasAssetOptIn := make(map[string]bool)

	for idx, stxn := range stxns {
		groupKey := groupKey(stxn.Txn.Group, idx)
		if isAssetOptInTxn(stxn.Txn) {
			groupHasAssetOptIn[groupKey] = true
		}
		if stxn.Txn.RekeyTo == (basics.Address{}) {
			continue
		}

		groupHasRekey[groupKey] = true
		report.Rekeys = append(report.Rekeys, RekeyTxnInfo{
			Index:        idx,
			Sender:       stxn.Txn.Sender,
			RekeyTo:      stxn.Txn.RekeyTo,
			Type:         stxn.Txn.Type,
			Group:        stxn.Txn.Group,
			AssetOptIn:   isAssetOptInTxn(stxn.Txn),
			SameGroupKey: groupKey,
		})
	}

	for _, txn := range report.Rekeys {
		if groupHasAssetOptIn[txn.SameGroupKey] && groupHasRekey[txn.SameGroupKey] {
			report.HasAssetOptInAndRekey = true
			break
		}
	}

	return report
}

func groupKey(group crypto.Digest, index int) string {
	if group == (crypto.Digest{}) {
		return fmt.Sprintf("ungrouped-%d", index)
	}
	return group.String()
}

func isAssetOptInTxn(txn transactions.Transaction) bool {
	return txn.Type == protocol.AssetTransferTx &&
		txn.XferAsset != 0 &&
		txn.AssetAmount == 0 &&
		txn.AssetSender == (basics.Address{}) &&
		txn.AssetReceiver == txn.Sender &&
		txn.AssetCloseTo == (basics.Address{})
}
