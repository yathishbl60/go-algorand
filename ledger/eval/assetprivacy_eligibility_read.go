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
	"github.com/algorand/go-algorand/internal/assetprivacy/eligibility"
)

// AssetPrivacyEligibilityRoot returns only current effective-freeze authorization.
// Never accept it through historical note roots; recheck after group mutations.
func (cs *roundCowState) AssetPrivacyEligibilityRoot(asset basics.AssetIndex) (eligibility.Root, uint64, error) {
	store, err := cs.openAssetPrivacyEligibility(asset)
	if err != nil {
		return eligibility.Root{}, 0, err
	}
	return store.Snapshot()
}

// GetAssetPrivacyEligibility authenticates a snapshot and rechecks native sources.
// Missing membership grants no eligibility. This is an internal local interface.
func (cs *roundCowState) GetAssetPrivacyEligibility(asset basics.AssetIndex, account basics.Address) (eligibility.Record, bool, error) {
	store, err := cs.openAssetPrivacyEligibility(asset)
	if err != nil {
		return eligibility.Record{}, false, err
	}
	record, exists, err := store.Current(account, uint64(cs.Round()))
	if err != nil || !exists {
		return eligibility.Record{}, exists, err
	}
	if err = cs.checkAssetPrivacyEligibility(record); err != nil {
		return eligibility.Record{}, true, err
	}
	return record, true, nil
}

func (cs *roundCowState) checkAssetPrivacyEligibility(record eligibility.Record) error {
	frozen, _, err := cs.AssetPrivacyFrozen(basics.AssetIndex(record.Domain.Asset), basics.Address(record.Domain.Account))
	if err != nil {
		return err
	}
	if frozen != record.Frozen {
		return eligibility.ErrHistory
	}
	return nil
}
