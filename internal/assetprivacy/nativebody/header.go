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

package nativebody

import (
	"encoding/binary"

	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func readHeader(encoded []byte) Body {
	body := Body{Operation: Kind(encoded[5]), Mode: policy.Modes(encoded[6]),
		Asset: binary.BigEndian.Uint64(encoded[7:15]), Debit: binary.BigEndian.Uint64(encoded[143:151]),
		Credit: binary.BigEndian.Uint64(encoded[183:191])}
	copy(body.Policy[:], encoded[15:79])
	copy(body.Suite[:], encoded[79:143])
	copy(body.DebitSource[:], encoded[151:183])
	copy(body.CreditReceiver[:], encoded[191:223])
	return body
}

func writeHeader(encoded []byte, body Body) {
	copy(encoded[:5], "NAPT\x01")
	encoded[5], encoded[6] = byte(body.Operation), byte(body.Mode)
	binary.BigEndian.PutUint64(encoded[7:15], body.Asset)
	copy(encoded[15:79], body.Policy[:])
	copy(encoded[79:143], body.Suite[:])
	binary.BigEndian.PutUint64(encoded[143:151], body.Debit)
	copy(encoded[151:183], body.DebitSource[:])
	binary.BigEndian.PutUint64(encoded[183:191], body.Credit)
	copy(encoded[191:223], body.CreditReceiver[:])
	encoded[223], encoded[224] = byte(len(body.Inputs)), byte(len(body.Outputs))
}
