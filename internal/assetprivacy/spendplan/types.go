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

// Package spendplan derives expected main-proof statements, not proof acceptance.
package spendplan

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/eligibility"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

var (
	ErrConfig    = errors.New("invalid native privacy spending proof configuration")
	ErrStatement = errors.New("invalid native privacy spending proof statement")
	ErrOutputs   = errors.New("native privacy output records do not match spending proof")
)

type Kind byte

const (
	Transfer Kind = 1 + iota
	Unshield
)

const (
	TransferPublicValues = 264
	UnshieldPublicValues = 196
)

// Input contains public nullifier/root and native accepted-root count metadata.
// Count is never added to the AIR vector; native root acceptance remains required.
type Input struct {
	Nullifier [64]byte
	Root      [64]byte
	Count     uint64
}

// Statement has exactly two actual private inputs. Outputs are actual commitments:
// two for Transfer, zero for Unshield. There are no optional/dummy input slots.
type Statement struct {
	Kind         Kind
	Inputs       [2]Input
	Outputs      [][64]byte
	Debit        uint64
	Credit       uint64
	Group        [64]byte
	Participants [4][32]byte
}

// Config comes from current native state, not proof-advertised values or roots.
type Config struct {
	Domain          tree.Domain
	EligibilityRoot eligibility.Root
}
