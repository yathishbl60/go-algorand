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

// LookupNativeAssetPrivacyWitness returns a public path, never note ownership.
func (l *Ledger) LookupNativeAssetPrivacyWitness(round basics.Round, asset basics.AssetIndex, index uint32) (query.NoteWitness, error) {
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	current, err := l.nativePrivacyOutputs(round, asset)
	if err != nil {
		return query.NoteWitness{}, err
	}
	witness, err := current.commitments.Witness(index)
	if err != nil {
		return query.NoteWitness{}, err
	}
	record, err := current.stream.Read(index)
	if err != nil {
		return query.NoteWitness{}, err
	}
	if record.Commitment != witness.Leaf {
		return query.NoteWitness{}, query.ErrState
	}
	return query.NoteWitness{Round: uint64(round), Asset: uint64(asset), PolicyID: current.snapshot.PolicyID, Witness: witness}, nil
}
