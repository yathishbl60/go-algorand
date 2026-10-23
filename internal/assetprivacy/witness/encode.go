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

// Encode returns an owned envelope, copying proof without retaining its storage.
// The caller must supply the independently selected profile and final group digest.
func Encode(proof []byte, binding Binding, limits Limits) ([]byte, error) {
	if err := validate(binding, limits); err != nil {
		return nil, err
	}
	if len(proof) == 0 || uint64(len(proof)) > uint64(limits.MaxProofBytes) {
		return nil, ErrLength
	}
	header := makeHeader(binding, uint32(len(proof)))
	encoded := make([]byte, HeaderSize+len(proof))
	copy(encoded, header[:])
	copy(encoded[HeaderSize:], proof)
	return encoded, nil
}
