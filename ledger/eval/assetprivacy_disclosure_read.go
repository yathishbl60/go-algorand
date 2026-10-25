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
	"github.com/algorand/go-algorand/internal/assetprivacy/disclosure"
)

// GetAssetPrivacyDisclosureTargets reads policy-bound current viewing assignments.
// It never grants issuer powers or accepts a caller-chosen target/root.
func (cs *roundCowState) GetAssetPrivacyDisclosureTargets(asset basics.AssetIndex) (disclosure.Record, bool, error) {
	store, err := cs.openAssetPrivacyDisclosureTargets(asset)
	if err != nil {
		return disclosure.Record{}, false, err
	}
	record, exists, err := store.Current(uint64(cs.Round()))
	if err != nil {
		return disclosure.Record{}, exists, err
	}
	if !exists && store.Required() != 0 {
		return disclosure.Record{}, false, disclosure.ErrMissing
	}
	if exists {
		if err = cs.checkDisclosureController(asset, record); err != nil {
			return disclosure.Record{}, true, err
		}
	}
	return record, exists, nil
}

func (cs *roundCowState) checkDisclosureController(asset basics.AssetIndex, record disclosure.Record) error {
	if record.Flags&disclosure.ControllerRequired == 0 {
		return nil
	}
	params, _, err := cs.AssetParams(asset)
	if err != nil {
		return err
	}
	if !params.Clawback.IsZero() && [32]byte(params.Clawback) != record.Controller {
		return disclosure.ErrTarget
	}
	return nil
}
