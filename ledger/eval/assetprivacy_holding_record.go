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

func (cs *roundCowState) recordPrivacyHoldingState(addr basics.Address, asset basics.AssetIndex, data ledgercore.AssetHoldingDelta, declaration policy.Record, before basics.AssetHolding, hadHolding bool) error {
	identity, err := declaration.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return err
	}
	domain := control.Domain{Network: cs.GenesisHash(), Asset: uint64(asset), Policy: identity, Account: addr}
	store, err := control.Open(assetPrivacyControlKV{cs}, domain)
	if err != nil {
		return err
	}
	marker, hasMarker, err := store.Current(uint64(cs.Round()))
	if err != nil {
		return err
	}
	frozen := privacyHoldingFrozen(data, before, hadHolding, marker, hasMarker)
	_, err = store.Set(frozen, marker.Epoch, uint64(cs.Round()))
	return err
}
