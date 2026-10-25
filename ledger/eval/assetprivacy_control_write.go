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
	"github.com/algorand/go-algorand/internal/assetprivacy/control"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// SetAssetPrivacyFrozen persists an action only after caller authentication.
// The authority argument is an identity check, not a signature or possession proof.
// Caller must verify full freeze/PQ authorization, state binding and storage fees.
func (cs *roundCowState) SetAssetPrivacyFrozen(asset basics.AssetIndex, account, authority basics.Address, expectedEpoch uint64, frozen bool) (control.Record, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return control.Record{}, policy.ErrDisabled
	}
	child := cs.child(0)
	defer child.recycle()
	record, err := child.setAssetPrivacyFrozenState(asset, account, authority, expectedEpoch, frozen)
	if err != nil {
		return control.Record{}, err
	}
	child.commitToParent()
	return record, nil
}
