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

package statement

import "encoding/binary"

// Values injectively encodes the group digest as 32 little-endian uint16 scalars.
// The complete AIR must include these in its public-value count, and the native
// verifier must recompute and provide them. Computing Bind alone is insufficient.
func Values(digest [64]byte) [32]uint16 {
	var values [32]uint16
	for i := range values {
		values[i] = binary.LittleEndian.Uint16(digest[2*i : 2*i+2])
	}
	return values
}
