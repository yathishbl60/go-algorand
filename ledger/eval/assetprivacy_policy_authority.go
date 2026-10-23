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

func (cs *roundCowState) privacyPolicyAuthority(asset basics.AssetIndex, manager basics.Address) (policy.ExistingAsset, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return policy.ExistingAsset{}, policy.ErrDisabled
	}
	params, _, err := cs.AssetParams(asset)
	if err != nil {
		return policy.ExistingAsset{}, err
	}
	if params.Manager.IsZero() || params.Manager != manager {
		return policy.ExistingAsset{}, policy.ErrManager
	}
	capabilities := policy.Capabilities(0)
	if !params.Freeze.IsZero() {
		capabilities |= policy.CanFreeze
	}
	if !params.Clawback.IsZero() {
		capabilities |= policy.CanClawback
	}
	return policy.ExistingAsset{Managed: true, Capabilities: capabilities}, nil
}

func (cs *roundCowState) writeAssetPrivacyPolicy(asset basics.AssetIndex, record policy.Record) error {
	key, err := policy.RecordKey(uint64(asset))
	if err != nil {
		return err
	}
	encoded, err := policy.EncodeRecord(record)
	if err != nil {
		return err
	}
	return cs.kvPut(key, encoded)
}
