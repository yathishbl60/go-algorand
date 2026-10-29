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
	"crypto/sha3"

	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

func nativePrivacyCacheIdentity(tx *transactions.SignedTxn) *privacyWitnessIdentity {
	if tx.Txn.Type != protocol.NativeAssetPrivacyTx || len(tx.NativeAssetPrivacyProof) == 0 {
		return nil
	}
	return &privacyWitnessIdentity{length: uint64(len(tx.NativeAssetPrivacyProof)),
		digest: sha3.Sum512(tx.NativeAssetPrivacyProof)}
}

func nativePrivacyCacheMatches(identity *privacyWitnessIdentity, tx *transactions.SignedTxn) bool {
	if tx.Txn.Type != protocol.NativeAssetPrivacyTx {
		return identity == nil && len(tx.NativeAssetPrivacyProof) == 0 && len(tx.Txn.NativeAssetPrivacy) == 0
	}
	if identity == nil || identity.length != uint64(len(tx.NativeAssetPrivacyProof)) {
		return false
	}
	current := nativePrivacyCacheIdentity(tx)
	return current != nil && *identity == *current
}
