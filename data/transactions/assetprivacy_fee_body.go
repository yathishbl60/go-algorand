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

package transactions

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/registrationproof"
)

func (s SignedTxn) nativeAssetPrivacyProofChecks() (uint64, error) {
	if lifecyclebody.IsLifecycle(s.Txn.NativeAssetPrivacy) {
		return s.nativeAssetPrivacyLifecycleChecks()
	}
	body, err := nativebody.Decode(s.Txn.NativeAssetPrivacy)
	if err != nil {
		return 0, err
	}
	return body.ProofChecks()
}

func (s SignedTxn) nativeAssetPrivacyLifecycleChecks() (uint64, error) {
	body, err := lifecyclebody.Decode(s.Txn.NativeAssetPrivacy)
	if err != nil {
		return 0, err
	}
	if body.Operation == lifecyclebody.Register {
		return s.nativeAssetPrivacyRegistrationChecks(body)
	}
	if len(s.NativeAssetPrivacyProof) != 0 {
		return 0, nativebody.ErrEncoding
	}
	if body.Operation == lifecyclebody.Cleanup {
		return 1, nil
	}
	return 0, nil
}

// Pricing alone grants no admission: the group checker binds the exact NAPW.
func (s SignedTxn) nativeAssetPrivacyRegistrationChecks(body lifecyclebody.Body) (uint64, error) {
	if body.Role.UsesStarkPossession() {
		if len(s.NativeAssetPrivacyProof) == 0 {
			return 0, nativebody.ErrEncoding
		}
		return 1, nil
	}
	return 1, registrationproof.Check(s.NativeAssetPrivacyProof)
}
