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

type assetPrivacyAuthorityChange struct {
	before, after basics.Address
	role          keys.Role
}

func assetPrivacyAuthorityChanges(before, after basics.AssetParams) [3]assetPrivacyAuthorityChange {
	return [3]assetPrivacyAuthorityChange{
		{before.Manager, after.Manager, keys.ManagerAuthority},
		{before.Freeze, after.Freeze, keys.FreezeAuthority},
		{before.Clawback, after.Clawback, keys.ClawbackAuthority},
	}
}

func (cs *roundCowState) requireAssetPrivacyAuthorityKeys(asset basics.AssetIndex, before, after basics.AssetParams, declaration policy.Record) error {
	for _, change := range assetPrivacyAuthorityChanges(before, after) {
		if change.after.IsZero() || change.after == change.before {
			continue
		}
		if err := cs.requireAssetPrivacyAuthorityKey(asset, change.after, change.role); err != nil {
			return err
		}
	}
	changedClawback := !after.Clawback.IsZero() && after.Clawback != before.Clawback
	if changedClawback && declaration.Policy.Disclosures&policy.ControllerDisclosure != 0 {
		return cs.requireAssetPrivacyAuthorityKey(asset, after.Clawback, keys.ControllerView)
	}
	return nil
}

func (cs *roundCowState) requireAssetPrivacyAuthorityKey(asset basics.AssetIndex, account basics.Address, role keys.Role) error {
	store, err := cs.openAssetPrivacyKeys(asset, account, role)
	if err != nil {
		return err
	}
	record, exists, err := store.Current(uint64(cs.Round()))
	if err != nil {
		return err
	}
	if !exists {
		return keys.ErrKey
	}
	if err = cs.checkAssetPrivacyKeyRole(record); err != nil {
		return err
	}
	_, err = cs.AssetPrivacyKeyAuthorizationWitness(record)
	return err
}
