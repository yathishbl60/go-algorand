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
)

// LookupNativeAssetPrivacyOutputCursor accepts no latest-round fallback.
func (l *Ledger) LookupNativeAssetPrivacyOutputCursor(encoded string, limit uint32) (query.OutputPage, error) {
	cursor, err := query.DecodeCursor(encoded)
	if err != nil {
		return query.OutputPage{}, err
	}
	if err = query.CheckPage(cursor.Next, limit); err != nil {
		return query.OutputPage{}, err
	}
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	current, err := l.nativePrivacyOutputs(basics.Round(cursor.Round), basics.AssetIndex(cursor.Asset))
	if err != nil {
		return query.OutputPage{}, err
	}
	if err = cursor.CheckSnapshot(current.snapshot, current.commitments.Root(), current.commitments.Count()); err != nil {
		return query.OutputPage{}, err
	}
	return current.page(cursor.Next, limit)
}

func (current nativePrivacyOutputQuery) page(start uint64, limit uint32) (query.OutputPage, error) {
	page, err := current.stream.Page(start, limit)
	if err != nil {
		return query.OutputPage{}, err
	}
	return query.OutputPage{Round: current.snapshot.Round, Asset: current.snapshot.Asset, PolicyID: current.snapshot.PolicyID,
		Root: current.commitments.Root(), Page: page}, nil
}
