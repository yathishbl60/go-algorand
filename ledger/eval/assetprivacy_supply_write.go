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

// PutAssetPrivacySupply records a checked aggregate in the current COW delta.
// Callers must also enforce proofs, policy, public balances and storage funding.
func (cs *roundCowState) PutAssetPrivacySupply(aidx basics.AssetIndex, amount uint64) error {
	supply, _, err := cs.GetAssetPrivacySupply(aidx)
	if err != nil {
		return err
	}
	supply.Private = amount
	if err = supply.Validate(); err != nil {
		return err
	}
	key, err := accounting.SupplyKey(uint64(aidx))
	if err != nil {
		return err
	}
	return cs.kvPut(key, accounting.EncodePrivateSupply(amount))
}

// DeleteAssetPrivacySupply removes only an empty supply record.
// It does not delete other privacy state or authorize destruction of the asset.
func (cs *roundCowState) DeleteAssetPrivacySupply(aidx basics.AssetIndex) error {
	supply, exists, err := cs.GetAssetPrivacySupply(aidx)
	if err != nil || !exists {
		return err
	}
	if supply.Private != 0 {
		return accounting.ErrOutstanding
	}
	key, err := accounting.SupplyKey(uint64(aidx))
	if err != nil {
		return err
	}
	return cs.kvDel(key)
}
