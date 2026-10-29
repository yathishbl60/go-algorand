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

	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
)

var errAssetPrivacyUpgrade = errors.New("native privacy upgrade requires configured execution and retained state compatibility")

func checkAssetPrivacyUpgrade(previous, next config.ConsensusParams) error {
	if !previous.EnableNativeAssetPrivacy && !next.EnableNativeAssetPrivacy {
		return nil
	}
	if !next.NativeAssetPrivacyExecutionConfigured() || next.NativeAssetPrivacyRootHistory == 0 {
		return errAssetPrivacyUpgrade
	}
	if err := suitecatalog.Validate(next); err != nil {
		return err
	}
	if previous.EnableNativeAssetPrivacy {
		return checkAssetPrivacyRetainedState(previous, next)
	}
	return nil
}

// Existing ring layouts and suite identities have no in-place migration yet.
func checkAssetPrivacyRetainedState(previous, next config.ConsensusParams) error {
	if previous.NativeAssetPrivacyRootHistory != next.NativeAssetPrivacyRootHistory {
		return errAssetPrivacyUpgrade
	}
	if err := suitecatalog.Validate(previous); err != nil {
		return err
	}
	for _, old := range previous.NativeAssetPrivacySuites {
		if !assetPrivacySuiteRetained(old, next.NativeAssetPrivacySuites) {
			return errAssetPrivacyUpgrade
		}
	}
	return nil
}

func assetPrivacySuiteRetained(old config.NativeAssetPrivacySuite, records []config.NativeAssetPrivacySuite) bool {
	for _, candidate := range records {
		if candidate == old {
			return true
		}
	}
	return false
}
