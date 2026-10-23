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

// Package statement binds the candidate proof context to finalized group bodies.
// It does not authorize transactions, verify proofs or replace consensus validation.
package statement

import "errors"

var ErrDomain = errors.New("privacy statement requires nonzero network and suite digests")
var ErrGroup = errors.New("invalid privacy proof group context")
var ErrLimit = errors.New("privacy proof context exceeds trusted encoding limits")

// Limits must come from trusted consensus/profile configuration, never a transaction.
// Encoding estimates are checked before any transaction is marshaled.
type Limits struct {
	MaxGroupSize     int
	MaxBodyEstimate  uint64
	MaxGroupEstimate uint64
}

func (l Limits) validate() error {
	if l.MaxGroupSize < 1 || l.MaxGroupSize > 65535 {
		return ErrLimit
	}
	if l.MaxBodyEstimate == 0 || l.MaxGroupEstimate < l.MaxBodyEstimate {
		return ErrLimit
	}
	return nil
}
