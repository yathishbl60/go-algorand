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

package recoveryplan

import "encoding/binary"

func publicValues(s Statement, c Config) []uint16 {
	values := make([]uint16, 0, PartialPrivatePublicValues)
	values = appendDigest(values, s.Input.Nullifier)
	values = appendDigest(values, s.Input.Root)
	for _, output := range s.Outputs {
		values = appendDigest(values, output)
	}
	if s.Kind.public() {
		values = appendAmount(values, s.Credit)
	}
	values = appendDigest(values, s.Group)
	for _, value := range c.Authority {
		values = append(values, uint16(value))
	}
	return values
}

func appendDigest(values []uint16, digest [64]byte) []uint16 {
	for offset := 0; offset < len(digest); offset += 2 {
		values = append(values, binary.LittleEndian.Uint16(digest[offset:offset+2]))
	}
	return values
}

func appendAmount(values []uint16, amount uint64) []uint16 {
	for limb := 0; limb < 4; limb++ {
		values = append(values, uint16(amount>>(16*limb)))
	}
	return values
}
