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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

type nativePrivacyAuthorizationQuery struct {
	snapshot query.Snapshot
	params   basics.AssetParams
	domain   tree.Domain
	kv       nativePrivacyQueryKV
}

func (l *Ledger) nativePrivacyAuthorizationContext(round basics.Round, asset basics.AssetIndex) (nativePrivacyAuthorizationQuery, error) {
	snapshot, err := l.nativePrivacySnapshot(round, asset)
	if err != nil {
		return nativePrivacyAuthorizationQuery{}, err
	}
	if err = query.CheckLiveState(snapshot); err != nil {
		return nativePrivacyAuthorizationQuery{}, err
	}
	params, err := l.nativePrivacyAuthorizationParams(snapshot)
	if err != nil {
		return nativePrivacyAuthorizationQuery{}, err
	}
	return nativePrivacyAuthorizationQuery{snapshot: snapshot, params: params, kv: nativePrivacyQueryKV{&l.accts, round},
		domain: tree.Domain{Genesis: l.GenesisHash(), Asset: uint64(asset), Policy: snapshot.PolicyID}}, nil
}

func (l *Ledger) nativePrivacyAuthorizationParams(snapshot query.Snapshot) (basics.AssetParams, error) {
	resource, _, err := l.accts.LookupResource(basics.Round(snapshot.Round), basics.Address(snapshot.Creator), basics.CreatableIndex(snapshot.Asset), basics.AssetCreatable)
	if err != nil {
		return basics.AssetParams{}, err
	}
	if resource.AssetParams == nil {
		return basics.AssetParams{}, query.ErrState
	}
	params := *resource.AssetParams
	if params.Total != snapshot.Total || params.Manager != snapshot.Manager || nativePrivacyQueryCapabilities(params) != snapshot.Policy.Policy.Capabilities {
		return basics.AssetParams{}, query.ErrState
	}
	return params, nil
}

func nativePrivacyQueryCapabilities(params basics.AssetParams) policy.Capabilities {
	var capabilities policy.Capabilities
	if !params.Freeze.IsZero() {
		capabilities |= policy.CanFreeze
	}
	if !params.Clawback.IsZero() {
		capabilities |= policy.CanClawback
	}
	return capabilities
}
