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

package verify

import (
	"errors"

	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

func nativePrivacyCacheIdentity(tx *transactions.SignedTxn) *privacyWitnessIdentity { return nil }

func nativePrivacyCacheMatches(identity *privacyWitnessIdentity, tx *transactions.SignedTxn) bool {
	return identity == nil && tx.Txn.Type != protocol.NativeAssetPrivacyTx &&
		len(tx.NativeAssetPrivacyProof) == 0 && len(tx.Txn.NativeAssetPrivacy) == 0
}

func checkNativePrivacyGroup(group []transactions.SignedTxn, proto config.ConsensusParams) error {
	for _, tx := range group {
		if !nativePrivacyCacheMatches(nil, &tx) {
			return errors.New("native asset privacy requires the feature build")
		}
	}
	return nil
}

func nativePrivacyDiagnostic(tx transactions.SignedTxn) any {
	if tx.Txn.Type == protocol.NativeAssetPrivacyTx || len(tx.NativeAssetPrivacyProof) != 0 {
		return tx.ID()
	}
	return tx
}
