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

func (cs *roundCowState) readAssetPrivacyClosure(asset basics.AssetIndex) (closure.Record, bool, error) {
	key, err := closure.Key(uint64(asset))
	if err != nil {
		return closure.Record{}, false, err
	}
	encoded, exists, err := cs.kvGet(key)
	if err != nil || !exists {
		return closure.Record{}, exists, err
	}
	record, err := closure.Decode(encoded)
	if err != nil {
		return closure.Record{}, true, err
	}
	if record.Created > uint64(cs.Round()) {
		return closure.Record{}, true, closure.ErrRecord
	}
	return record, true, nil
}

func (cs *roundCowState) writeAssetPrivacyClosure(asset basics.AssetIndex, record closure.Record) error {
	key, err := closure.Key(uint64(asset))
	if err != nil {
		return err
	}
	encoded, err := closure.Encode(record)
	if err != nil {
		return err
	}
	return cs.kvPut(key, encoded)
}

func (cs *roundCowState) assetPrivacyExists(asset basics.AssetIndex) (bool, error) {
	_, exists, err := cs.GetCreator(basics.CreatableIndex(asset), basics.AssetCreatable)
	return exists, err
}

func (cs *roundCowState) requireAssetPrivacyUnclosed(asset basics.AssetIndex) error {
	_, closed, err := cs.readAssetPrivacyClosure(asset)
	if err != nil {
		return err
	}
	if closed {
		return closure.ErrRecord
	}
	return nil
}

func (cs *roundCowState) requireAssetPrivacyAbsent(asset basics.AssetIndex) error {
	exists, err := cs.assetPrivacyExists(asset)
	if err != nil {
		return err
	}
	if exists {
		return closure.ErrRecord
	}
	return nil
}
