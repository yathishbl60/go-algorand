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
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func (cs *roundCowState) checkAssetPrivacyKeyRole(record keys.Record) error {
	asset := basics.AssetIndex(record.Domain.Asset)
	declaration, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return err
	}
	if !exists {
		return policy.ErrMissing
	}
	if err = record.Domain.Role.CheckPolicy(declaration); err != nil {
		return err
	}
	params, _, err := cs.AssetParams(asset)
	if err != nil {
		return err
	}
	return checkPrivacyKeyAccount(record.Domain, params)
}

func checkPrivacyKeyAccount(domain keys.Domain, params basics.AssetParams) error {
	var expected basics.Address
	switch domain.Role {
	case keys.ManagerAuthority:
		expected = params.Manager
	case keys.FreezeAuthority:
		expected = params.Freeze
	case keys.ClawbackAuthority, keys.ControllerView:
		expected = params.Clawback
	default:
		return nil
	}
	if expected.IsZero() || [32]byte(expected) != domain.Account {
		return keys.ErrAuthority
	}
	return nil
}
