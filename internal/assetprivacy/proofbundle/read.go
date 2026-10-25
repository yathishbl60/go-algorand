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

package proofbundle

import "encoding/binary"

type reader struct{ remaining []byte }

func (r *reader) next(slot Slot, limits Limits) ([]byte, error) {
	if len(r.remaining) < EntrySize || !slot.matches(r.remaining[:EntrySize]) {
		return nil, ErrFrame
	}
	length := uint64(binary.BigEndian.Uint32(r.remaining[6:10]))
	if length > uint64(len(r.remaining)-EntrySize) || length > uint64(limits.MaxProofBytes) {
		return nil, ErrFrame
	}
	proof := r.remaining[EntrySize : EntrySize+int(length)]
	if err := limits.checkProof(proof); err != nil {
		return nil, err
	}
	r.remaining = r.remaining[EntrySize+int(length):]
	return proof, nil
}

// Check validates every descriptor and length without decoding or accepting proofs.
// It requires the entire exact set before any owned payload copy is allocated.
func Check(encoded []byte, plan *Plan, limits Limits) error {
	if err := checkHeader(encoded, plan, limits); err != nil {
		return err
	}
	r := reader{remaining: encoded[HeaderSize:]}
	for _, slot := range plan.slots {
		if _, err := r.next(slot, limits); err != nil {
			return err
		}
	}
	if len(r.remaining) != 0 {
		return ErrFrame
	}
	return nil
}
