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

package witness

import (
	"fmt"
	"io"
)

// Read requires a finite transaction frame, and returns an owned opaque proof.
// It checks the fixed header and trusted bounds before allocating proof storage.
// Strict trailing-byte rejection requires reading EOF after the declared payload.
func Read(reader io.Reader, expected Binding, limits Limits) ([]byte, error) {
	if err := validate(expected, limits); err != nil {
		return nil, err
	}
	var header [HeaderSize]byte
	if err := readFull(reader, header[:]); err != nil {
		return nil, err
	}
	length, err := parseHeader(header[:], expected, limits)
	if err != nil {
		return nil, err
	}
	return readProof(reader, length)
}

func readProof(reader io.Reader, length uint32) ([]byte, error) {
	proof := make([]byte, int(length))
	if err := readFull(reader, proof); err != nil {
		return nil, err
	}
	if err := requireEOF(reader); err != nil {
		return nil, err
	}
	return proof, nil
}

func readFull(reader io.Reader, target []byte) error {
	if _, err := io.ReadFull(reader, target); err != nil {
		if err == io.EOF || err == io.ErrUnexpectedEOF {
			return fmt.Errorf("%w: %w", ErrTruncated, err)
		}
		return fmt.Errorf("%w: %w", ErrIO, err)
	}
	return nil
}

func requireEOF(reader io.Reader) error {
	var extra [1]byte
	if n, err := io.ReadFull(reader, extra[:]); n != 0 {
		return ErrTrailing
	} else if err != io.EOF {
		return fmt.Errorf("%w: %w", ErrIO, err)
	}
	return nil
}
