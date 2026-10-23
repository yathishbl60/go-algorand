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

package policy

// Mode describes the visibility of a transfer, not its wire encoding.
type Mode uint8

const (
	// Public exposes participants and amounts.
	Public Mode = 1 << iota
	// Confidential exposes participants while hiding private amounts.
	Confidential
	// Shielded hides private owners and note values.
	Shielded
)

// Modes is the set of transfer modes permitted by an asset's immutable policy.
type Modes uint8

// AllModes includes each supported visibility mode.
const AllModes = Modes(Public | Confidential | Shielded)

// Allows accepts exactly one known mode included in a valid mode set.
func (m Modes) Allows(mode Mode) bool {
	if m&^AllModes != 0 {
		return false
	}
	switch mode {
	case Public, Confidential, Shielded:
		return m&Modes(mode) != 0
	default:
		return false
	}
}
