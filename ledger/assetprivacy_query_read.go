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

package ledger

import (
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

type nativePrivacyQueryKV struct {
	accounts *accountUpdates
	round    basics.Round
}

// Tracker lookup buffers never escape; decoders must return owned records.
func (kv nativePrivacyQueryKV) Get(key string) ([]byte, bool, error) {
	encoded, err := kv.accounts.LookupKv(kv.round, key)
	return encoded, encoded != nil, err
}

func (kv nativePrivacyQueryKV) Apply(_ []tree.Record) error { return query.ErrReadOnly }

func (l *Ledger) nativePrivacyQueryConfig(round basics.Round, asset basics.AssetIndex) (query.Config, error) {
	config := query.Config{Network: l.GenesisHash(), Asset: uint64(asset), Round: uint64(round)}
	creator, exists, err := l.accts.GetCreatorForRound(round, basics.CreatableIndex(asset), basics.AssetCreatable)
	if err != nil {
		return config, err
	}
	config.Exists = exists
	if !exists {
		return config, nil
	}
	return l.loadNativePrivacyQueryConfig(round, asset, creator, config)
}

func (l *Ledger) loadNativePrivacyQueryConfig(round basics.Round, asset basics.AssetIndex, creator basics.Address, config query.Config) (query.Config, error) {
	resource, _, err := l.accts.LookupResource(round, creator, basics.CreatableIndex(asset), basics.AssetCreatable)
	if err != nil {
		return config, err
	}
	if resource.AssetParams == nil {
		return config, query.ErrState
	}
	config.Creator, config.Manager, config.Total = creator, resource.AssetParams.Manager, resource.AssetParams.Total
	if resource.AssetHolding != nil {
		config.CreatorHolding, config.HasCreatorHolding = resource.AssetHolding.Amount, true
	}
	return config, nil
}
