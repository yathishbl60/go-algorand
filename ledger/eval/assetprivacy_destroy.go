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
	"errors"

	"github.com/algorand/go-algorand/data/basics"
)

var errAssetPrivacyCleanup = errors.New("native privacy asset destruction requires authenticated state cleanup")

// CheckAssetPrivacyDestruction requires the authenticated finalizer to remove
// every native record before the ordinary applier removes creator attribution.
func (cs *roundCowState) CheckAssetPrivacyDestruction(asset basics.AssetIndex, holding uint64) error {
	supply, exists, err := cs.GetAssetPrivacySupply(asset)
	if err != nil {
		return err
	}
	if err = supply.CheckDestruction(holding); err != nil {
		return err
	}
	if exists {
		return errAssetPrivacyCleanup
	}
	return cs.checkAssetPrivacyPolicyCleanup(asset)
}

func (cs *roundCowState) checkAssetPrivacyPolicyCleanup(asset basics.AssetIndex) error {
	_, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return err
	}
	if exists {
		return errAssetPrivacyCleanup
	}
	return cs.checkAssetPrivacyBookkeepingCleanup(asset)
}
