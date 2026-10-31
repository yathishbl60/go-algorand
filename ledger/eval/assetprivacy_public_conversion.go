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
	"fmt"

	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// Caller verifies complete proofs, current authorization and funding in a child COW.
// Recovery grants only the already-authorized public-credit freeze exception.
func (cs *roundCowState) applyAssetPrivacyPublicConversion(value AssetPrivacyPublicConversion, recovery bool) error {
	if !cs.proto.EnableNativeAssetPrivacy {
		return policy.ErrDisabled
	}
	if recovery && value.Debit != 0 {
		return fmt.Errorf("native privacy recovery cannot debit a public holding")
	}
	if err := cs.assetPrivacyPublicDebit(value.Asset, value.DebitSource, value.Debit); err != nil {
		return err
	}
	return cs.assetPrivacyPublicCredit(value.Asset, value.CreditReceiver, value.Credit, recovery)
}

func (cs *roundCowState) assetPrivacyPublicHolding(asset basics.AssetIndex, address basics.Address, bypassFreeze bool) (basics.AssetHolding, error) {
	if address.IsZero() {
		return basics.AssetHolding{}, fmt.Errorf("native privacy public conversion requires a nonzero account")
	}
	holding, exists, err := cs.GetAssetHolding(address, asset)
	if err != nil {
		return basics.AssetHolding{}, err
	}
	if !exists {
		return basics.AssetHolding{}, fmt.Errorf("asset %d missing from %s: public conversion requires opt-in", asset, address)
	}
	if err = cs.checkAssetPrivacyConversionFrozen(asset, address, bypassFreeze); err != nil {
		return basics.AssetHolding{}, err
	}
	return holding, nil
}
