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

// Decode validates the complete frame before allocating an owned proof copy.
// It never returns a slice backed by encoded; it does not decode the STARK.
func Decode(encoded []byte, expected Binding, limits Limits) ([]byte, error) {
	if err := validate(expected, limits); err != nil {
		return nil, err
	}
	if len(encoded) < HeaderSize {
		return nil, ErrTruncated
	}
	length, err := parseHeader(encoded[:HeaderSize], expected, limits)
	if err != nil {
		return nil, err
	}
	if err = checkFrameLength(uint64(len(encoded)), uint64(HeaderSize)+uint64(length)); err != nil {
		return nil, err
	}
	return append([]byte(nil), encoded[HeaderSize:]...), nil
}

func checkFrameLength(actual, expected uint64) error {
	if actual < expected {
		return ErrTruncated
	}
	if actual > expected {
		return ErrTrailing
	}
	return nil
}
