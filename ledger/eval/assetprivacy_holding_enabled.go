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

func (cs *roundCowState) putAssetHoldingWithPrivacy(addr basics.Address, asset basics.AssetIndex, data ledgercore.AssetHoldingDelta) error {
	if !cs.proto.EnableNativeAssetPrivacy {
		return cs.putAssetHolding(addr, asset, data)
	}
	declaration, exists, err := cs.privacyHoldingPolicy(asset)
	if err != nil {
		return err
	}
	if !exists {
		return cs.putAssetHolding(addr, asset, data)
	}
	return cs.putPrivacyHoldingAtomic(addr, asset, data, declaration)
}

func (cs *roundCowState) putPrivacyHoldingAtomic(addr basics.Address, asset basics.AssetIndex, data ledgercore.AssetHoldingDelta, declaration policy.Record) error {
	if err := cs.checkAssetPrivacyActive(asset); err != nil {
		return err
	}
	child := cs.child(0)
	defer child.recycle()
	if err := child.synchronizePrivacyHolding(addr, asset, data, declaration); err != nil {
		return err
	}
	if err := child.putAssetHolding(addr, asset, data); err != nil {
		return err
	}
	if err := child.reconcilePrivacyHolding(addr, asset, data, declaration); err != nil {
		return err
	}
	child.commitToParent()
	return nil
}
