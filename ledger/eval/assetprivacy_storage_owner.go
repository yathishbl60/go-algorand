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
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/storage"
)

func (cs *roundCowState) assetPrivacyStorageOwner(asset uint64) (basics.Address, error) {
	_, creator, err := cs.AssetParams(basics.AssetIndex(asset))
	if err != nil {
		return basics.Address{}, err
	}
	if creator.IsZero() || creator == cs.mods.Hdr.FeeSink || creator == cs.mods.Hdr.RewardsPool || creator == transactions.StateProofSender {
		return basics.Address{}, storage.ErrPayer
	}
	return creator, nil
}

func (cs *roundCowState) assetPrivacyNextObligation(payer basics.Address, key string, before []byte, exists bool, after []byte, present bool) (storage.Obligation, error) {
	value, found, err := cs.assetPrivacyStorageObligation(payer)
	if err != nil {
		return storage.Obligation{}, err
	}
	if !found {
		value, err = storage.New(payer)
		if err != nil {
			return storage.Obligation{}, err
		}
	}
	return value.Change(uint64(len(key)), uint64(len(before)), exists, uint64(len(after)), present)
}
