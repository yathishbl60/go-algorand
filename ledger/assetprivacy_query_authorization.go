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
	"github.com/algorand/go-algorand/internal/assetprivacy/eligibility"
	"github.com/algorand/go-algorand/internal/assetprivacy/keyauth"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (l *Ledger) LookupNativeAssetPrivacyAuthorization(round basics.Round, asset basics.AssetIndex) (query.AuthorizationSnapshot, error) {
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	context, err := l.nativePrivacyAuthorizationContext(round, asset)
	if err != nil {
		return query.AuthorizationSnapshot{}, err
	}
	return context.authorization()
}

func (context nativePrivacyAuthorizationQuery) authorization() (query.AuthorizationSnapshot, error) {
	result := query.AuthorizationSnapshot{Round: context.snapshot.Round, Asset: context.snapshot.Asset, PolicyID: context.snapshot.PolicyID,
		Manager: context.params.Manager, Freeze: context.params.Freeze, Clawback: context.params.Clawback, DefaultFrozen: context.params.DefaultFrozen}
	targets, err := context.targets()
	if err != nil {
		return query.AuthorizationSnapshot{}, err
	}
	result.Targets = targets
	return context.authorizationRoots(result)
}

func (context nativePrivacyAuthorizationQuery) authorizationRoots(result query.AuthorizationSnapshot) (query.AuthorizationSnapshot, error) {
	keys, err := keyauth.Open(context.kv, context.domain)
	if err != nil {
		return query.AuthorizationSnapshot{}, err
	}
	result.Keys, result.KeyCount, err = keys.Snapshot()
	if err != nil {
		return query.AuthorizationSnapshot{}, err
	}
	eligible, err := eligibility.Open(context.kv, context.domain)
	if err != nil {
		return query.AuthorizationSnapshot{}, err
	}
	result.Eligibility, result.EligibilityCount, err = eligible.Snapshot()
	if err != nil {
		return query.AuthorizationSnapshot{}, err
	}
	return result, nil
}
