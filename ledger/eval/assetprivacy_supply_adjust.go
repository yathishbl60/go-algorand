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

import "github.com/algorand/go-algorand/data/basics"

// IncreaseAssetPrivateSupply applies only the accounting leg of a shield.
// The eventual transaction handler must commit the matching public debit.
func (cs *roundCowState) IncreaseAssetPrivateSupply(aidx basics.AssetIndex, amount uint64) error {
	supply, _, err := cs.GetAssetPrivacySupply(aidx)
	if err != nil {
		return err
	}
	next, err := supply.Shield(amount)
	if err != nil {
		return err
	}
	return cs.PutAssetPrivacySupply(aidx, next.Private)
}

// DecreaseAssetPrivateSupply applies only the accounting leg of an unshield.
// The eventual transaction handler must commit the matching public credit.
func (cs *roundCowState) DecreaseAssetPrivateSupply(aidx basics.AssetIndex, amount uint64) error {
	supply, _, err := cs.GetAssetPrivacySupply(aidx)
	if err != nil {
		return err
	}
	next, err := supply.Unshield(amount)
	if err != nil {
		return err
	}
	return cs.PutAssetPrivacySupply(aidx, next.Private)
}
