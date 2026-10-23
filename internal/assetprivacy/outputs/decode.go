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

package outputs

import (
	"encoding/binary"
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// Decode checks exact bounded length before allocating any envelope copies.
func Decode(encoded []byte) (Record, error) {
	if len(encoded) < headerSize || encoded[0] != 1 {
		return Record{}, ErrEncoding
	}
	flags := policy.Disclosures(encoded[249])
	size, err := encodedSize(flags)
	if err != nil || len(encoded) != size {
		return Record{}, ErrEncoding
	}
	r := decodeHeader(encoded)
	decodeEnvelopes(&r, encoded)
	return r, r.Validate()
}

func decodeEnvelopes(r *Record, encoded []byte) {
	r.Recipient = append([]byte(nil), encoded[headerSize:headerSize+envelope.EncodedSize]...)
	tail := encoded[headerSize+envelope.EncodedSize:]
	if r.Disclosures&policy.AuditorDisclosure != 0 {
		r.Auditor = append([]byte(nil), tail[:envelope.EncodedSize]...)
		tail = tail[envelope.EncodedSize:]
	}
	if r.Disclosures&policy.ControllerDisclosure != 0 {
		r.Controller = append([]byte(nil), tail...)
	}
}

func decodeHeader(encoded []byte) Record {
	r := Record{Domain: readDomain(encoded[1:105]), Index: binary.BigEndian.Uint32(encoded[105:109]),
		Round: binary.BigEndian.Uint64(encoded[109:117]), Output: binary.BigEndian.Uint32(encoded[181:185]),
		Disclosures: policy.Disclosures(encoded[249])}
	copy(r.Transaction[:], encoded[117:149])
	copy(r.Nonce[:], encoded[149:181])
	copy(r.Commitment[:], encoded[185:249])
	return r
}
