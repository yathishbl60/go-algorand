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

package shieldplan

import "encoding/binary"

// The vector matches ShieldAir: outputs64, debit4 and group32 uint16 cells.
func publicValues(group [64]byte, debit uint64, outputs [2][64]byte) [PublicValues]uint16 {
	var values [PublicValues]uint16
	for index, commitment := range outputs {
		putDigest(values[index*32:index*32+32], commitment)
	}
	for limb := 0; limb < 4; limb++ {
		values[64+limb] = uint16(debit >> (16 * limb))
	}
	putDigest(values[68:], group)
	return values
}

func putDigest(values []uint16, digest [64]byte) {
	for limb := 0; limb < 32; limb++ {
		values[limb] = binary.LittleEndian.Uint16(digest[2*limb : 2*limb+2])
	}
}
