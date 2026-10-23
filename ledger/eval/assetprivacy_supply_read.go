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
	"github.com/algorand/go-algorand/internal/assetprivacy/accounting"
)

// GetAssetPrivacySupply reads the current group-local supply and authoritative total.
// Absence means no aggregate record, not an approved privacy policy.
func (cs *roundCowState) GetAssetPrivacySupply(aidx basics.AssetIndex) (accounting.Supply, bool, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return accounting.Supply{}, false, accounting.ErrDisabled
	}
	params, _, err := cs.AssetParams(aidx)
	if err != nil {
		return accounting.Supply{}, false, err
	}
	amount, exists, err := cs.readAssetPrivacySupply(aidx)
	if err != nil {
		return accounting.Supply{}, exists, err
	}
	supply := accounting.Supply{Total: params.Total, Private: amount}
	return supply, exists, supply.Validate()
}

func (cs *roundCowState) readAssetPrivacySupply(aidx basics.AssetIndex) (uint64, bool, error) {
	key, err := accounting.SupplyKey(uint64(aidx))
	if err != nil {
		return 0, false, err
	}
	encoded, exists, err := cs.kvGet(key)
	if err != nil || !exists {
		return 0, exists, err
	}
	amount, err := accounting.DecodePrivateSupply(encoded)
	return amount, true, err
}
