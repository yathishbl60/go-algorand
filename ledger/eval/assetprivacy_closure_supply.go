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
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
)

func (cs *roundCowState) checkAssetPrivacyFinalSupply(asset basics.AssetIndex, creator basics.Address, credit uint64) error {
	supply, exists, err := cs.GetAssetPrivacySupply(asset)
	if err != nil {
		return err
	}
	if !exists || credit == 0 || supply.Private != credit {
		return closure.ErrRecord
	}
	holding, exists, err := cs.GetAssetHolding(creator, asset)
	if err != nil {
		return err
	}
	if !exists || holding.Amount != supply.Total-credit {
		return closure.ErrRecord
	}
	return cs.requireAssetPrivacyUnclosed(asset)
}
