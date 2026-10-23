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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// AdoptAssetPrivacyPolicy preserves public mode and current issuer rights.
// Caller must verify ordinary/PQ manager authorization, pinned suite and funding.
func (cs *roundCowState) AdoptAssetPrivacyPolicy(asset basics.AssetIndex, manager basics.Address, record policy.Record) error {
	existing, err := cs.privacyPolicyAuthority(asset, manager)
	if err != nil {
		return err
	}
	_, existing.HasPolicy, err = cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return err
	}
	if record.Created != uint64(cs.Round()) || record.InitialCapabilities != record.Policy.Capabilities {
		return policy.ErrRecord
	}
	if err = policy.CheckAdoption(true, record.Policy, existing); err != nil {
		return err
	}
	return cs.writeAssetPrivacyPolicy(asset, record)
}

// UpdateAssetPrivacyPolicy permits only relinquishment matching current ASA keys.
// Caller must authenticate manager/PQ keys and apply related key changes atomically.
func (cs *roundCowState) UpdateAssetPrivacyPolicy(asset basics.AssetIndex, manager basics.Address, after policy.Record) error {
	existing, err := cs.privacyPolicyAuthority(asset, manager)
	if err != nil {
		return err
	}
	before, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return err
	}
	if !exists {
		return policy.ErrMissing
	}
	if after.Policy.Capabilities != existing.Capabilities {
		return policy.ErrCapabilities
	}
	if err = policy.CheckRecordUpdate(true, before, after); err != nil {
		return err
	}
	return cs.writeAssetPrivacyPolicy(asset, after)
}
