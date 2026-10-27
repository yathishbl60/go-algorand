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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
)

// GetAssetPrivacyPolicy returns canonical group-local policy state, not authority proof.
func (cs *roundCowState) GetAssetPrivacyPolicy(asset basics.AssetIndex) (policy.Record, bool, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return policy.Record{}, false, policy.ErrDisabled
	}
	if _, _, err := cs.AssetParams(asset); err != nil {
		return policy.Record{}, false, err
	}
	key, err := policy.RecordKey(uint64(asset))
	if err != nil {
		return policy.Record{}, false, err
	}
	encoded, exists, err := cs.kvGet(key)
	if err != nil || !exists {
		return policy.Record{}, exists, err
	}
	record, err := cs.decodeAssetPrivacyPolicy(encoded)
	return record, true, err
}

func (cs *roundCowState) decodeAssetPrivacyPolicy(encoded []byte) (policy.Record, error) {
	record, err := policy.DecodeRecord(encoded)
	if err != nil {
		return policy.Record{}, err
	}
	if record.Created > uint64(cs.Round()) {
		return policy.Record{}, policy.ErrRecord
	}
	if _, err = suitecatalog.Lookup(cs.proto, record.Suite); err != nil {
		return policy.Record{}, err
	}
	return record, nil
}
