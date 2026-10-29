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

package lifecyclebody

import "encoding/binary"

func readHeader(encoded []byte) Body {
	body := Body{Operation: Kind(encoded[5]), Asset: binary.BigEndian.Uint64(encoded[6:14]),
		ExpectedEpoch: binary.BigEndian.Uint64(encoded[142:150])}
	copy(body.Policy[:], encoded[14:78])
	copy(body.Suite[:], encoded[78:142])
	return body
}

func writeHeader(encoded []byte, body Body) {
	copy(encoded[:5], "NAPL\x01")
	encoded[5] = byte(body.Operation)
	binary.BigEndian.PutUint64(encoded[6:14], body.Asset)
	copy(encoded[14:78], body.Policy[:])
	copy(encoded[78:142], body.Suite[:])
	binary.BigEndian.PutUint64(encoded[142:150], body.ExpectedEpoch)
}
