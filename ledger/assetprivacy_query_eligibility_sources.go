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
	"github.com/algorand/go-algorand/internal/assetprivacy/control"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (l *Ledger) nativePrivacyEligibilitySources(context nativePrivacyAuthorizationQuery, account basics.Address) (query.EligibilitySnapshot, error) {
	result := query.EligibilitySnapshot{Round: context.snapshot.Round, Asset: context.snapshot.Asset, PolicyID: context.snapshot.PolicyID, Account: account}
	domain := control.Domain{Network: context.domain.Genesis, Asset: context.domain.Asset, Policy: context.domain.Policy, Account: account}
	store, err := control.Open(query.ReadOnly[control.Write]{Reader: context.kv}, domain)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	marker, hasMarker, err := store.Current(context.snapshot.Round)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	result.Frozen, err = l.nativePrivacyEffectiveFrozen(context, account, marker, hasMarker)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	if hasMarker {
		result.Marker = &marker
	}
	return result, nil
}

func (l *Ledger) nativePrivacyEffectiveFrozen(context nativePrivacyAuthorizationQuery, account basics.Address, marker control.Record, hasMarker bool) (bool, error) {
	resource, _, err := l.accts.LookupResource(basics.Round(context.snapshot.Round), account, basics.CreatableIndex(context.snapshot.Asset), basics.AssetCreatable)
	if err != nil {
		return false, err
	}
	hasHolding := resource.AssetHolding != nil
	holdingFrozen := hasHolding && resource.AssetHolding.Frozen
	return control.EffectiveFrozen(marker, hasMarker, hasHolding, holdingFrozen, context.params.DefaultFrozen), nil
}
