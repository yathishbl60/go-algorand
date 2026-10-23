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
	"github.com/algorand/go-algorand/internal/assetprivacy/nullifier"
)

// HasAssetPrivacyNullifier reads current group-local consumption state.
// Errors, including malformed records, must never be interpreted as unspent.
func (cs *roundCowState) HasAssetPrivacyNullifier(asset basics.AssetIndex, marker [64]byte) (bool, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return false, nullifier.ErrDisabled
	}
	key, err := nullifier.Key(uint64(asset), marker)
	if err != nil {
		return false, err
	}
	if _, _, err = cs.AssetParams(asset); err != nil {
		return false, err
	}
	encoded, exists, err := cs.kvGet(key)
	if err != nil || !exists {
		return exists, err
	}
	return true, nullifier.ValidateRecord(encoded)
}

// ConsumeAssetPrivacyNullifier rejects reuse and records consumption in this COW.
// The caller must validate the complete proof, state, funding and balance changes,
// and discard the transaction/group COW on any later failure.
func (cs *roundCowState) ConsumeAssetPrivacyNullifier(asset basics.AssetIndex, marker [64]byte) error {
	exists, err := cs.HasAssetPrivacyNullifier(asset, marker)
	if err != nil {
		return err
	}
	if exists {
		return nullifier.ErrSpent
	}
	key, err := nullifier.Key(uint64(asset), marker)
	if err != nil {
		return err
	}
	return cs.kvPut(key, nullifier.EncodeSpent())
}
