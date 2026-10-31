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
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

func (cs *roundCowState) readAssetPrivacyRetirement(asset basics.AssetIndex) (retirement.Record, bool, error) {
	key, err := retirement.Key(uint64(asset))
	if err != nil {
		return retirement.Record{}, false, err
	}
	encoded, exists, err := cs.kvGet(key)
	if err != nil || !exists {
		return retirement.Record{}, exists, err
	}
	record, err := retirement.Decode(encoded)
	if err != nil {
		return retirement.Record{}, true, err
	}
	if record.Created > uint64(cs.Round()) {
		return retirement.Record{}, true, retirement.ErrRecord
	}
	return record, true, nil
}

func (cs *roundCowState) checkAssetPrivacyActive(asset basics.AssetIndex) error {
	_, retired, err := cs.readAssetPrivacyRetirement(asset)
	if err != nil {
		return err
	}
	if retired {
		return retirement.ErrRetired
	}
	return nil
}
