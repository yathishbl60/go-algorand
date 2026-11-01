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
	"github.com/algorand/go-algorand/internal/assetprivacy/notetree"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

type nativePrivacyOutputQuery struct {
	snapshot    query.Snapshot
	stream      *outputs.Store
	commitments *notetree.Store
}

func (l *Ledger) nativePrivacyOutputs(round basics.Round, asset basics.AssetIndex) (nativePrivacyOutputQuery, error) {
	snapshot, err := l.nativePrivacySnapshot(round, asset)
	if err != nil {
		return nativePrivacyOutputQuery{}, err
	}
	if err = query.CheckOutputState(snapshot); err != nil {
		return nativePrivacyOutputQuery{}, err
	}
	domain := tree.Domain{Genesis: l.GenesisHash(), Asset: uint64(asset), Policy: snapshot.PolicyID}
	return l.openNativePrivacyOutputQuery(snapshot, domain)
}

func (l *Ledger) openNativePrivacyOutputQuery(snapshot query.Snapshot, domain tree.Domain) (nativePrivacyOutputQuery, error) {
	kv := nativePrivacyQueryKV{&l.accts, basics.Round(snapshot.Round)}
	stream, err := outputs.Open(kv, domain, snapshot.Policy.Policy.Disclosures)
	if err != nil {
		return nativePrivacyOutputQuery{}, err
	}
	commitments, err := notetree.Open(kv, domain)
	if err != nil {
		return nativePrivacyOutputQuery{}, err
	}
	if stream.Count() != commitments.Count() || stream.LastRound() > snapshot.Round {
		return nativePrivacyOutputQuery{}, query.ErrState
	}
	return nativePrivacyOutputQuery{snapshot, stream, commitments}, nil
}

// LookupNativeAssetPrivacyOutputs never advances the requested round or cursor.
func (l *Ledger) LookupNativeAssetPrivacyOutputs(round basics.Round, asset basics.AssetIndex, start uint64, limit uint32) (query.OutputPage, error) {
	if err := query.CheckPage(start, limit); err != nil {
		return query.OutputPage{}, err
	}
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	current, err := l.nativePrivacyOutputs(round, asset)
	if err != nil {
		return query.OutputPage{}, err
	}
	return current.page(start, limit)
}
