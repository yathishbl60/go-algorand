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

import "encoding/binary"

func makeHeader(binding Binding, length uint32) [HeaderSize]byte {
	var header [HeaderSize]byte
	copy(header[:4], magic)
	header[4] = version
	copy(header[5:69], binding.Profile[:])
	copy(header[69:133], binding.Statement[:])
	binary.BigEndian.PutUint32(header[133:], length)
	return header
}

func parseHeader(header []byte, expected Binding, limits Limits) (uint32, error) {
	if err := validate(expected, limits); err != nil {
		return 0, err
	}
	if len(header) != HeaderSize {
		return 0, ErrTruncated
	}
	if string(header[:4]) != magic {
		return 0, ErrMagic
	}
	if header[4] != version {
		return 0, ErrVersion
	}
	return boundHeader(header, expected, limits)
}

func boundHeader(header []byte, expected Binding, limits Limits) (uint32, error) {
	if [64]byte(header[5:69]) != expected.Profile || [64]byte(header[69:133]) != expected.Statement {
		return 0, ErrBinding
	}
	length := binary.BigEndian.Uint32(header[133:])
	if length == 0 || length > limits.MaxProofBytes {
		return 0, ErrLength
	}
	return length, nil
}

func validate(binding Binding, limits Limits) error {
	if err := limits.validate(); err != nil {
		return err
	}
	return binding.validate()
}
