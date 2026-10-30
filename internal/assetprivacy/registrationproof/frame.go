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

import (
	"encoding/binary"

	"github.com/algorand/go-algorand/crypto"
)

// Check validates framing only, not public-key syntax or signature possession.
// Unknown proof kinds cannot fall back to another verification construction.
func Check(frame []byte) error {
	if len(frame) < HeaderSize || string(frame[:6]) != "NAPR\x01\x01" {
		return ErrFrame
	}
	size := int(binary.BigEndian.Uint16(frame[6:8]))
	if size == 0 || size > crypto.Falcon1024MaxSignatureSize || len(frame) != HeaderSize+size {
		return ErrFrame
	}
	return nil
}

// Encode owns its result. Signature generation and authority checks are separate.
func Encode(signature []byte) ([]byte, error) {
	if len(signature) == 0 || len(signature) > crypto.Falcon1024MaxSignatureSize {
		return nil, ErrFrame
	}
	frame := append([]byte(nil), "NAPR\x01\x01"...)
	frame = binary.BigEndian.AppendUint16(frame, uint16(len(signature)))
	return append(frame, signature...), nil
}

// Decode checks the complete bounded frame before copying an owned signature.
// Keep frame immutable until return. The returned bytes do not alias frame.
func Decode(frame []byte) ([]byte, error) {
	if err := Check(frame); err != nil {
		return nil, err
	}
	return append([]byte(nil), frame[HeaderSize:]...), nil
}
