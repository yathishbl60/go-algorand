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

package suitecatalog

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// Lookup validates every record before selecting, with no default or fallback suite.
// Keep proto.NativeAssetPrivacySuites stable during its bounded snapshot copy.
func Lookup(proto config.ConsensusParams, id [64]byte) (Suite, error) {
	records, count, err := snapshot(proto)
	if err != nil {
		return Suite{}, err
	}
	for _, record := range records[:count] {
		if record.ID == id {
			return Suite{record: record}, nil
		}
	}
	return Suite{}, ErrUnknown
}

// Validate checks complete metadata only; it performs no artifact hashing or I/O.
// An enabled empty catalog is valid but cannot select any suite.
func Validate(proto config.ConsensusParams) error {
	_, _, err := snapshot(proto)
	return err
}

func snapshot(proto config.ConsensusParams) ([MaxSuites]config.NativeAssetPrivacySuite, int, error) {
	var records [MaxSuites]config.NativeAssetPrivacySuite
	if !proto.EnableNativeAssetPrivacy {
		return records, 0, policy.ErrDisabled
	}
	if len(proto.NativeAssetPrivacySuites) > MaxSuites {
		return records, 0, ErrCatalog
	}
	count := copy(records[:], proto.NativeAssetPrivacySuites)
	if err := validateRecords(records[:count]); err != nil {
		return records, 0, err
	}
	return records, count, nil
}
