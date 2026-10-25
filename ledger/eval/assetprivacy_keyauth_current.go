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
	"bytes"

	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

func (cs *roundCowState) checkCurrentAssetPrivacyKey(record keys.Record) error {
	store, err := cs.OpenAssetPrivacyKeys(basics.AssetIndex(record.Domain.Asset), basics.Address(record.Domain.Account), record.Domain.Role)
	if err != nil {
		return err
	}
	current, exists, err := store.Current(uint64(cs.Round()))
	if err != nil {
		return err
	}
	if !exists || current.Domain != record.Domain || current.Epoch != record.Epoch || current.Created != record.Created {
		return keys.ErrHistory
	}
	if !bytes.Equal(current.PublicKey, record.PublicKey) {
		return keys.ErrHistory
	}
	return nil
}
