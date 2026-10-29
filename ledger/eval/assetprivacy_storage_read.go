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
	"github.com/algorand/go-algorand/internal/assetprivacy/storage"
)

func (cs *roundCowState) assetPrivacyStorageObligation(payer basics.Address) (storage.Obligation, bool, error) {
	key, err := storage.Key(payer)
	if err != nil {
		return storage.Obligation{}, false, err
	}
	encoded, exists, err := cs.kvGet(key)
	if err != nil || !exists {
		return storage.Obligation{}, exists, err
	}
	value, err := storage.Decode(encoded)
	return value, true, err
}

func (cs *roundCowState) assetPrivacyStorageMinimum(payer basics.Address) (uint64, error) {
	if !cs.proto.EnableNativeAssetPrivacy || payer.IsZero() {
		return 0, nil
	}
	value, exists, err := cs.assetPrivacyStorageObligation(payer)
	if err != nil || !exists {
		return 0, err
	}
	return value.Required(cs.proto.NativeAssetPrivacyFlatMinBalance, cs.proto.NativeAssetPrivacyByteMinBalance)
}
