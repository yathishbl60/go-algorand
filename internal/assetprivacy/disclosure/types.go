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

package disclosure

// Domain binds assignments to the immutable network/asset/policy identity.
type Domain struct {
	Network [32]byte
	Asset   uint64
	Policy  [64]byte
}

type Flags byte

const (
	AuditorRequired Flags = 1 << iota
	ControllerRequired
)

// Record assigns viewing targets only; it does not grant issuer powers.
type Record struct {
	Domain     Domain
	Flags      Flags
	Auditor    [32]byte
	Controller [32]byte
	Epoch      uint64
	Updated    uint64
}

type Write struct {
	Key   string
	Value []byte
}

// KV requires serialized access and atomic Apply or rollback on error.
type KV interface {
	Get(string) ([]byte, bool, error)
	Apply([]Write) error
}

type Store struct {
	kv     KV
	domain Domain
	flags  Flags
}

func (s *Store) Required() Flags { return s.flags }
