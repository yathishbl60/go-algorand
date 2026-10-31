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

package verify

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
	"github.com/algorand/go-algorand/protocol"
)

func checkNativePrivacyGroup(group []transactions.SignedTxn, proto config.ConsensusParams) error {
	for index := range group {
		tx := &group[index]
		if tx.Txn.Type != protocol.NativeAssetPrivacyTx && !tx.Txn.IsNativeAssetPrivacyCreation() {
			if len(tx.NativeAssetPrivacyProof) != 0 {
				return errNativePrivacyWitness
			}
			continue
		}
		limits, err := checkNativePrivacySigned(tx, proto)
		if err != nil {
			return err
		}
		if err = checkNativePrivacyBinding(group, index, limits, proto); err != nil {
			return err
		}
	}
	return nil
}

func nativePrivacyBodies(group []transactions.SignedTxn, proto config.ConsensusParams) ([]transactions.Transaction, statement.Limits, error) {
	if proto.MaxTxnBytesPerBlock <= 0 || proto.MaxTxGroupSize <= 0 ||
		len(group) == 0 || len(group) > proto.MaxTxGroupSize {
		return nil, statement.Limits{}, statement.ErrLimit
	}
	bodies := make([]transactions.Transaction, len(group))
	for i := range group {
		bodies[i] = group[i].Txn
	}
	limit := uint64(proto.MaxTxnBytesPerBlock)
	return bodies, statement.Limits{MaxGroupSize: proto.MaxTxGroupSize,
		MaxBodyEstimate: limit, MaxGroupEstimate: limit}, nil
}
