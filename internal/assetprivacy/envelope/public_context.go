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

package envelope

// PublicBytes returns the exact 248-byte verifier context with the key epoch omitted.
// Epoch is deliberately ignored; it remains an input to private AAD constraints.
func (c Context) PublicBytes() ([248]byte, error) {
	var public [248]byte
	c.Epoch = 1 // Validate public fields without requiring knowledge of the private epoch.
	full, err := c.Bytes()
	if err != nil {
		return public, err
	}
	copy(public[:147], full[:147])
	copy(public[147:], full[155:])
	return public, nil
}
