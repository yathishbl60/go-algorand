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
	"github.com/algorand/go-algorand/internal/assetprivacy/keyauth"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (l *Ledger) LookupNativeAssetPrivacyKey(round basics.Round, asset basics.AssetIndex, account basics.Address, role keys.Role) (query.KeySnapshot, error) {
	if account.IsZero() || role < keys.RecipientView || role > keys.ManagerAuthority {
		return query.KeySnapshot{}, query.ErrConfig
	}
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	context, err := l.nativePrivacyAuthorizationContext(round, asset)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	return context.key(account, role)
}

func (context nativePrivacyAuthorizationQuery) key(account basics.Address, role keys.Role) (query.KeySnapshot, error) {
	targets, err := context.targets()
	if err != nil {
		return query.KeySnapshot{}, err
	}
	allowed := role.CheckPolicy(*context.snapshot.Policy) == nil
	result := query.KeySnapshot{Round: context.snapshot.Round, Asset: context.snapshot.Asset, PolicyID: context.snapshot.PolicyID,
		Account: account, Role: role, PolicyAllowed: allowed, Assigned: allowed && context.assigned(account, role, targets)}
	registry, err := keyauth.Open(context.kv, context.domain)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	result.Root, result.Count, err = registry.Snapshot()
	if err != nil {
		return query.KeySnapshot{}, err
	}
	return context.keyRecord(result, registry)
}
