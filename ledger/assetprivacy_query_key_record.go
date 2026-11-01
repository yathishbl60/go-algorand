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
	"github.com/algorand/go-algorand/internal/assetprivacy/keyauth"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (context nativePrivacyAuthorizationQuery) keyRecord(result query.KeySnapshot, registry *keyauth.Store) (query.KeySnapshot, error) {
	domain := keys.Domain{Network: context.domain.Genesis, Asset: context.domain.Asset, Policy: context.domain.Policy,
		Account: result.Account, Role: result.Role}
	store, err := keys.Open(query.ReadOnly[keys.Write]{Reader: context.kv}, domain)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	record, exists, err := store.Current(context.snapshot.Round)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	if !exists {
		return result, nil
	}
	return context.keyWitness(result, registry, record)
}

func (context nativePrivacyAuthorizationQuery) keyWitness(result query.KeySnapshot, registry *keyauth.Store, record keys.Record) (query.KeySnapshot, error) {
	witness, err := registry.Witness(record, context.snapshot.Round)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	if witness.Root != result.Root.Digest || witness.Count != result.Count {
		return query.KeySnapshot{}, query.ErrState
	}
	result.Record, result.Witness = &record, &witness
	return result, nil
}
