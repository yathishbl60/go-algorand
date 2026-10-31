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
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/registrationproof"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
)

// Supported lifecycle operations use their own exact witness contract.
func nativePrivacySignedSuite(tx *transactions.SignedTxn) ([64]byte, error) {
	if lifecyclebody.IsLifecycle(tx.Txn.NativeAssetPrivacy) {
		return nativePrivacyLifecycleSuite(tx)
	}
	body, err := nativebody.Decode(tx.Txn.NativeAssetPrivacy)
	return body.Suite, err
}

func nativePrivacyLifecycleSuite(tx *transactions.SignedTxn) ([64]byte, error) {
	body, err := lifecyclebody.Decode(tx.Txn.NativeAssetPrivacy)
	if err != nil {
		return [64]byte{}, err
	}
	if (body.Operation == lifecyclebody.Create) != tx.Txn.IsNativeAssetPrivacyCreation() {
		return [64]byte{}, lifecyclebody.ErrOperation
	}
	if err = nativePrivacyLifecycleWitness(tx, body); err != nil {
		return [64]byte{}, err
	}
	return body.Suite, body.CheckSender([32]byte(tx.Txn.Sender))
}

// Framing is stateless admission; only ledger execution verifies possession.
func nativePrivacyLifecycleWitness(tx *transactions.SignedTxn, body lifecyclebody.Body) error {
	if body.Operation == lifecyclebody.Register {
		if body.Role.UsesFalconPossession() {
			return registrationproof.Check(tx.NativeAssetPrivacyProof)
		}
		if len(tx.NativeAssetPrivacyProof) == 0 {
			return errNativePrivacyWitness
		}
		return nil
	}
	if len(tx.NativeAssetPrivacyProof) != 0 {
		return errNativePrivacyWitness
	}
	return nil
}

// Lifecycle signatures still commit the complete ordinary group. No proof is
// implied by computing this digest; it preserves the same group/domain checks.
func nativePrivacyGroupBinding(group []transactions.SignedTxn, index int, proto config.ConsensusParams) ([64]byte, [64]byte, error) {
	bodies, bounds, err := nativePrivacyBodies(group, proto)
	if err != nil {
		return [64]byte{}, [64]byte{}, err
	}
	suite, err := nativePrivacySignedSuite(&group[index])
	if err != nil {
		return [64]byte{}, [64]byte{}, err
	}
	digest, err := statement.Bind([32]byte(group[index].Txn.GenesisHash), suite, uint32(index), bodies, bounds)
	return digest, suite, err
}
