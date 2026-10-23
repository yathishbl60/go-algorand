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

// Package note defines candidate private note openings and public hash outputs.
// A valid opening is not proof of membership, ownership or spending authority.
package note

import "errors"

// EncodedSize is the fixed plaintext envelope size; unused bytes must be zero.
const EncodedSize = 512
const contentSize = 241
const version byte = 1

var ErrNote = errors.New("invalid private note opening")
var ErrDomain = errors.New("private note requires a nonzero network domain")
var ErrSecret = errors.New("private spending secret must be nonzero")

// Note is secret wallet/prover material. Do not log it or expose it over node RPC.
type Note struct {
	Asset    uint64
	Policy   [64]byte
	Owner    [64]byte
	Account  [32]byte
	Value    uint64
	Serial   [32]byte
	Blinding [32]byte
}

// Validate checks the candidate opening shape, not authorization or existence.
func (n Note) Validate() error {
	if n.Asset == 0 || n.Policy == [64]byte{} || n.Owner == [64]byte{} {
		return ErrNote
	}
	if n.Account == [32]byte{} || n.Serial == [32]byte{} || n.Blinding == [32]byte{} {
		return ErrNote
	}
	return nil
}
