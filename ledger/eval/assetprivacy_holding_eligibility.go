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

// Runs after the holding write in the same COW, including unchanged Frozen writes.
func (cs *roundCowState) reconcilePrivacyHolding(addr basics.Address, asset basics.AssetIndex, data ledgercore.AssetHoldingDelta, declaration policy.Record) error {
	marker, exists, err := cs.privacyHoldingMarker(addr, asset, declaration)
	if err != nil {
		return err
	}
	if data.Deleted && !exists {
		return nil // A real prior holding deletion always persisted an explicit marker.
	}
	frozen := marker.Frozen
	if !data.Deleted {
		if data.Holding == nil {
			return control.ErrRecord
		}
		frozen = data.Holding.Frozen || (exists && marker.Frozen)
	}
	return cs.reconcilePrivacyHoldingSnapshot(addr, asset, declaration, frozen)
}

func (cs *roundCowState) privacyHoldingMarker(addr basics.Address, asset basics.AssetIndex, declaration policy.Record) (control.Record, bool, error) {
	identity, err := declaration.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return control.Record{}, false, err
	}
	domain := control.Domain{Network: cs.GenesisHash(), Asset: uint64(asset), Policy: identity, Account: addr}
	store, err := control.Open(assetPrivacyControlKV{cs}, domain)
	if err != nil {
		return control.Record{}, false, err
	}
	return store.Current(uint64(cs.Round()))
}

func (cs *roundCowState) reconcilePrivacyHoldingSnapshot(addr basics.Address, asset basics.AssetIndex, declaration policy.Record, frozen bool) error {
	store, err := cs.openPrivacyEligibilityDeclaration(asset, declaration)
	if err != nil {
		return err
	}
	_, err = store.Reconcile(addr, frozen, uint64(cs.Round()))
	return err
}
