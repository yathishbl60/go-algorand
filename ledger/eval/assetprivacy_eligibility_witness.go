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
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// AssetPrivacyEligibilityWitness supplies an exact-current local proving path.
// It exposes no public account-query endpoint or proof-verification result.
func (cs *roundCowState) AssetPrivacyEligibilityWitness(record eligibility.Record) (tree.Witness, error) {
	asset := basics.AssetIndex(record.Domain.Asset)
	current, exists, err := cs.GetAssetPrivacyEligibility(asset, basics.Address(record.Domain.Account))
	if err != nil {
		return tree.Witness{}, err
	}
	if !exists || current != record {
		return tree.Witness{}, eligibility.ErrHistory
	}
	store, err := cs.openAssetPrivacyEligibility(asset)
	if err != nil {
		return tree.Witness{}, err
	}
	return store.Witness(record, uint64(cs.Round()))
}
