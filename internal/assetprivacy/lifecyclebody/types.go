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

// Package lifecyclebody implements canonical candidate lifecycle bodies.
// Parsing never authenticates a principal, verifies a proof or mutates state.
package lifecyclebody

import (
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

const HeaderSize = 150

type Kind byte

// Register accepts owner commitments, ML-KEM viewing and administrative Falcon keys.
const (
	Adopt    Kind = 1
	Register Kind = 2
	Assign   Kind = 3
	Freeze   Kind = 4
	Create   Kind = 5
	Retire   Kind = 6
	Cleanup  Kind = 7
)

var ErrEncoding = errors.New("invalid native lifecycle body encoding")
var ErrOperation = errors.New("unsupported native lifecycle operation")
var ErrHeader = errors.New("invalid native lifecycle body domain")
var ErrFields = errors.New("noncanonical native lifecycle operation fields")
var ErrSender = errors.New("invalid native lifecycle sender or registration account")

// Body carries public candidate operation fields only. Unused fields must be zero.
type Body struct {
	Operation     Kind
	Asset         uint64
	Policy        [64]byte
	Suite         [64]byte
	ExpectedEpoch uint64
	Modes         policy.Modes
	Capabilities  policy.Capabilities
	Disclosures   policy.Disclosures
	Auditor       [32]byte
	Target        [32]byte
	Frozen        bool
	Role          keys.Role
	Account       [32]byte
	PublicKey     []byte
	CleanupLimit  uint16
}

func (operation Kind) encodedSize(role keys.Role) int {
	switch operation {
	case Adopt, Create:
		return HeaderSize + 35
	case Register:
		return registrationBodySize(role)
	case Assign:
		return HeaderSize + 32
	case Freeze:
		return HeaderSize + 33
	case Retire:
		return HeaderSize
	case Cleanup:
		return HeaderSize + 2
	default:
		return 0
	}
}
