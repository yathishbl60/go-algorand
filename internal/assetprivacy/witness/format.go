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

// Package witness frames an opaque candidate proof without decoding the STARK.
package witness

// HeaderSize is the fixed byte count preceding the opaque proof bytes.
const HeaderSize = 4 + 1 + 64 + 64 + 4
const version byte = 1
const magic = "NAPW"

// Binding must be supplied by trusted profile selection and group recomputation.
// Profile is the pinned candidate profile digest; Statement is the group digest.
type Binding struct {
	Profile   [64]byte
	Statement [64]byte
}

// Limits comes from trusted profile/consensus configuration, never the witness.
// It bounds only the outer byte envelope, not decoded STARK allocation or work.
type Limits struct {
	MaxProofBytes uint32
}

func (l Limits) validate() error {
	maxInt := uint64(^uint(0) >> 1)
	if l.MaxProofBytes == 0 || uint64(l.MaxProofBytes)+HeaderSize > maxInt {
		return ErrLimits
	}
	return nil
}

func (b Binding) validate() error {
	if b.Profile == [64]byte{} {
		return ErrBinding
	}
	return nil
}
