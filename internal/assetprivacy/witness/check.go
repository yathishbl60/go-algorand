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

// Check validates framing/binding without copying or decoding the proof payload.
// Keep encoded stable for the call; this establishes no cryptographic validity.
func Check(encoded []byte, expected Binding, limits Limits) error {
	if len(encoded) < HeaderSize {
		return ErrTruncated
	}
	length, err := parseHeader(encoded[:HeaderSize], expected, limits)
	if err != nil {
		return err
	}
	return checkFrameLength(uint64(len(encoded)), uint64(HeaderSize)+uint64(length))
}
