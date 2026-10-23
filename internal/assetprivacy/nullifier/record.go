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

// Package nullifier defines the candidate native spent-marker record encoding.
package nullifier

import (
	"encoding/binary"
	"errors"
)

var ErrDisabled = errors.New("native asset privacy is disabled")
var ErrAsset = errors.New("private spent marker requires a nonzero asset ID")
var ErrSpent = errors.New("private note nullifier has already been consumed")
var ErrEncoding = errors.New("invalid private spent-marker encoding")

// Key separates spent markers by asset in the native, non-AVM namespace.
// Every 64-byte hash output is valid, including the all-zero output.
func Key(asset uint64, marker [64]byte) (string, error) {
	if asset == 0 {
		return "", ErrAsset
	}
	encoded := binary.BigEndian.AppendUint64([]byte("aprv:n:"), asset)
	return string(append(encoded, marker[:]...)), nil
}

// EncodeSpent marks consumption; there is no serialized unspent record.
func EncodeSpent() []byte {
	return []byte{1}
}

// ValidateRecord rejects malformed or unknown-version records, never treating them as unspent.
func ValidateRecord(encoded []byte) error {
	if len(encoded) != 1 || encoded[0] != 1 {
		return ErrEncoding
	}
	return nil
}
