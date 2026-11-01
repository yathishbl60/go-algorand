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

package query

import (
	"encoding/base64"
	"encoding/binary"
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

var ErrCursor = errors.New("invalid or mismatched native privacy output cursor")

const cursorBytes = 161
const cursorChars = 215

type Cursor struct {
	Round, Asset   uint64
	PolicyID, Root [64]byte
	Count, Next    uint64
}

func (c Cursor) validate() error {
	if c.Asset == 0 || c.PolicyID == [64]byte{} || c.Next > c.Count || c.Count > tree.Capacity {
		return ErrCursor
	}
	return nil
}

func (c Cursor) Encode() (string, error) {
	if err := c.validate(); err != nil {
		return "", err
	}
	var encoded [cursorBytes]byte
	encoded[0] = 1
	binary.BigEndian.PutUint64(encoded[1:9], c.Round)
	binary.BigEndian.PutUint64(encoded[9:17], c.Asset)
	copy(encoded[17:81], c.PolicyID[:])
	copy(encoded[81:145], c.Root[:])
	binary.BigEndian.PutUint64(encoded[145:153], c.Count)
	binary.BigEndian.PutUint64(encoded[153:161], c.Next)
	return base64.RawURLEncoding.EncodeToString(encoded[:]), nil
}

func DecodeCursor(encoded string) (Cursor, error) {
	if len(encoded) != cursorChars {
		return Cursor{}, ErrCursor
	}
	value, err := base64.RawURLEncoding.Strict().DecodeString(encoded)
	if err != nil || len(value) != cursorBytes || value[0] != 1 {
		return Cursor{}, ErrCursor
	}
	result := Cursor{Round: binary.BigEndian.Uint64(value[1:9]), Asset: binary.BigEndian.Uint64(value[9:17]),
		Count: binary.BigEndian.Uint64(value[145:153]), Next: binary.BigEndian.Uint64(value[153:161])}
	copy(result.PolicyID[:], value[17:81])
	copy(result.Root[:], value[81:145])
	canonical, err := result.Encode()
	if err != nil || canonical != encoded {
		return Cursor{}, ErrCursor
	}
	return result, nil
}
