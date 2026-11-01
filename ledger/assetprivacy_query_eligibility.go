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
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (l *Ledger) LookupNativeAssetPrivacyEligibility(round basics.Round, asset basics.AssetIndex, account basics.Address) (query.EligibilitySnapshot, error) {
	if account.IsZero() {
		return query.EligibilitySnapshot{}, query.ErrConfig
	}
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	context, err := l.nativePrivacyAuthorizationContext(round, asset)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	result, err := l.nativePrivacyEligibilitySources(context, account)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	return context.eligibility(result)
}

func (context nativePrivacyAuthorizationQuery) eligibility(result query.EligibilitySnapshot) (query.EligibilitySnapshot, error) {
	store, err := eligibility.Open(context.kv, context.domain)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	result.Root, result.Count, err = store.Snapshot()
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	record, exists, err := store.Current(result.Account, context.snapshot.Round)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	if !exists {
		return result, nil
	}
	return context.eligibilityWitness(result, store, record)
}

func (context nativePrivacyAuthorizationQuery) eligibilityWitness(result query.EligibilitySnapshot, store *eligibility.Store, record eligibility.Record) (query.EligibilitySnapshot, error) {
	if record.Frozen != result.Frozen {
		return query.EligibilitySnapshot{}, eligibility.ErrHistory
	}

	witness, err := store.Witness(record, context.snapshot.Round)
	if err != nil {
		return query.EligibilitySnapshot{}, err
	}
	if witness.Root != result.Root.Digest || witness.Count != result.Count {
		return query.EligibilitySnapshot{}, query.ErrState
	}
	result.Record, result.Witness = &record, &witness
	return result, nil
}
