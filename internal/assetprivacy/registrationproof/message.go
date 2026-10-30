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

package registrationproof

// Message returns the exact typed bytes signed by the NEW registration key.
// group must be reconstructed with statement.Bind from the complete final group.
func Message(group [64]byte) ([]byte, error) {
	if group == [64]byte{} {
		return nil, ErrGroup
	}
	message := append([]byte(nil), messageDomain...)
	return append(message, group[:]...), nil
}
