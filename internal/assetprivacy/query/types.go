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

package query

import (
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
	"github.com/algorand/go-algorand/internal/assetprivacy/inventory"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

var ErrReadOnly = errors.New("native privacy query cannot write ledger state")
var ErrUnavailable = errors.New("native privacy outputs are unavailable after retirement or destruction")

var ErrConfig = errors.New("invalid native privacy snapshot configuration")
var ErrState = errors.New("inconsistent native privacy snapshot state")
var ErrNotFound = errors.New("asset and native privacy closure do not exist")

type Reader interface {
	Get(string) ([]byte, bool, error)
}

// Config must describe authoritative ASA parameters at the same round as Reader.
type Config struct {
	Network             [32]byte
	Asset, Round, Total uint64
	Creator, Manager    [32]byte
	Exists              bool
	CreatorHolding      uint64
	HasCreatorHolding   bool
}

// Snapshot contains owned public records, never private note openings or secrets.
type Snapshot struct {
	Round, Asset, Total uint64
	Creator, Manager    [32]byte
	Exists              bool
	CreatorHolding      uint64
	HasCreatorHolding   bool
	Policy              *policy.Record
	PolicyID            [64]byte
	Private             uint64
	Retired             *retirement.Record
	Closure             *closure.Record
	Inventory           *inventory.Record
}

type state struct {
	Snapshot
	supplyPresent bool
}

func (c Config) validate() error {
	if c.Network == [32]byte{} || c.Asset == 0 || c.Exists && c.Creator == [32]byte{} {
		return ErrConfig
	}
	if !c.Exists && (c.Creator != [32]byte{} || c.Manager != [32]byte{} || c.Total != 0 || c.CreatorHolding != 0 || c.HasCreatorHolding) {
		return ErrConfig
	}
	if c.HasCreatorHolding && c.CreatorHolding > c.Total {
		return ErrConfig
	}
	return nil
}
