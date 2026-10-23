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

package note

import "encoding/binary"

// Encode returns canonical fixed-size plaintext with a zero-filled reserved tail.
func (n Note) Encode() ([]byte, error) {
	if err := n.Validate(); err != nil {
		return nil, err
	}
	out := make([]byte, EncodedSize)
	out[0] = version
	binary.BigEndian.PutUint64(out[1:9], n.Asset)
	copy(out[9:73], n.Policy[:])
	copy(out[73:137], n.Owner[:])
	copy(out[137:169], n.Account[:])
	binary.BigEndian.PutUint64(out[169:177], n.Value)
	copy(out[177:209], n.Serial[:])
	copy(out[209:241], n.Blinding[:])
	return out, nil
}
