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
)

func (s SignedTxn) nativeAssetPrivacyProofChecks() (uint64, error) {
	if lifecyclebody.IsLifecycle(s.Txn.NativeAssetPrivacy) {
		if len(s.NativeAssetPrivacyProof) != 0 {
			return 0, nativebody.ErrEncoding
		}
		_, err := lifecyclebody.Decode(s.Txn.NativeAssetPrivacy)
		return 0, err
	}
	body, err := nativebody.Decode(s.Txn.NativeAssetPrivacy)
	if err != nil {
		return 0, err
	}
	return body.ProofChecks()
}
