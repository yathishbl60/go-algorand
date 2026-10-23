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
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

// Synchronization projects already-authorized public holding state into controls.
// It does not authenticate public/PQ transactions or authorize a new operation.
// Ordinary freeze ingress still needs the required native PQ authorization.
// Unchanged balance writes keep independent markers; opt-in only adds restrictions.
// An actual public Frozen transition synchronizes both states and advances epoch.
// An authorized ordinary unfreeze therefore clears the corresponding native freeze.
// Deletion retains public.Frozen OR marker.Frozen, including an explicit false
// marker when needed to preserve eligibility against a default-frozen fallback.
func (cs *roundCowState) synchronizePrivacyHolding(addr basics.Address, asset basics.AssetIndex, data ledgercore.AssetHoldingDelta, declaration policy.Record) error {
	before, exists, err := cs.GetAssetHolding(addr, asset)
	if err != nil {
		return err
	}
	if data.Deleted && !exists {
		return nil
	}
	if !data.Deleted && data.Holding == nil {
		return control.ErrRecord
	}
	if !data.Deleted && exists && before.Frozen == data.Holding.Frozen {
		return nil
	}
	return cs.recordPrivacyHoldingState(addr, asset, data, declaration, before, exists)
}

func privacyHoldingFrozen(data ledgercore.AssetHoldingDelta, before basics.AssetHolding, hadHolding bool, marker control.Record, hasMarker bool) bool {
	if data.Deleted {
		return before.Frozen || (hasMarker && marker.Frozen)
	}
	if !hadHolding {
		return data.Holding.Frozen || (hasMarker && marker.Frozen)
	}
	return data.Holding.Frozen
}
