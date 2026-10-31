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
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// A policy without Public can only originate at native creation. Its creator's
// ordinary initial allocation is inventory, convertible only by a full Shield.
func (cs *roundCowState) checkAssetPrivacyIssuerInventory(current policy.Policy, body nativebody.Body) error {
	if err := cs.checkAssetPrivacyDebitState(body); err != nil {
		return err
	}
	return cs.checkAssetPrivacyIssuerSource(current, body)
}

func (cs *roundCowState) checkAssetPrivacyIssuerSource(current policy.Policy, body nativebody.Body) error {
	if current.Modes.Allows(policy.Public) || body.Debit == 0 {
		return nil
	}
	if body.Operation != nativebody.Shield || body.Credit != 0 {
		return policy.ErrModes
	}
	_, creator, err := cs.AssetParams(basics.AssetIndex(body.Asset))
	if err != nil {
		return err
	}
	if basics.Address(body.DebitSource) != creator {
		return policy.ErrModes
	}
	return nil
}
