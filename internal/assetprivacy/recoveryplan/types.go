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

// Package recoveryplan derives expected clawback statements, not authorization.
package recoveryplan

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

var (
	ErrConfig    = errors.New("invalid native privacy recovery proof configuration")
	ErrStatement = errors.New("invalid native privacy recovery proof statement")
	ErrOutputs   = errors.New("native privacy output records do not match recovery proof")
	ErrAuthority = errors.New("native privacy recovery requires retained current clawback authority")
)

type Kind byte

const (
	PartialPrivate Kind = 1 + iota
	FullPrivate
	PartialPublic
	FullPublic
)

const (
	PartialPrivatePublicValues = 192
	FullPrivatePublicValues    = 160
	PartialPublicPublicValues  = 164
	FullPublicPublicValues     = 132
)

// Statement consumes one actual input. Count is accepted-root native metadata.
// Private variants use actual output commitments; public variants credit an amount.
type Statement struct {
	Kind    Kind
	Input   spendplan.Input
	Outputs [][64]byte
	Credit  uint64
	Group   [64]byte
}

// Authority is the current ASA clawback account, not a view target or auth proof.
// Native code must derive Domain/Authority from authoritative current state.
type Config struct {
	Domain    tree.Domain
	Authority [32]byte
}
