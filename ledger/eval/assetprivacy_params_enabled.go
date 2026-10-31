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
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

func (cs *roundCowState) putAssetParamsWithPrivacy(addr basics.Address, asset basics.AssetIndex, data basics.AssetParams) error {
	if !cs.proto.EnableNativeAssetPrivacy {
		return cs.putAssetParams(addr, asset, ledgercore.AssetParamsDelta{Params: &data})
	}
	declaration, exists, err := cs.privacyHoldingPolicy(asset)
	if err != nil {
		return err
	}
	if !exists {
		return cs.putAssetParams(addr, asset, ledgercore.AssetParamsDelta{Params: &data})
	}
	return cs.putPrivacyAssetParamsAtomic(addr, asset, data, declaration)
}

func (cs *roundCowState) putPrivacyAssetParamsAtomic(addr basics.Address, asset basics.AssetIndex, after basics.AssetParams, declaration policy.Record) error {
	if err := cs.checkAssetPrivacyActive(asset); err != nil {
		return err
	}
	before, err := cs.assetPrivacyParamsBefore(addr, asset)
	if err != nil {
		return err
	}
	child := cs.child(0)
	defer child.recycle()
	if err = child.synchronizePrivacyAssetParams(asset, before, after, declaration); err != nil {
		return err
	}
	if err = child.putAssetParams(addr, asset, ledgercore.AssetParamsDelta{Params: &after}); err != nil {
		return err
	}
	child.commitToParent()
	return nil
}

func (cs *roundCowState) assetPrivacyParamsBefore(addr basics.Address, asset basics.AssetIndex) (basics.AssetParams, error) {
	before, creator, err := cs.AssetParams(asset)
	if err != nil {
		return basics.AssetParams{}, err
	}
	if creator != addr || before.Manager.IsZero() {
		return basics.AssetParams{}, policy.ErrManager
	}
	return before, nil
}
