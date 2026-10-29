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

package artifactstore

import (
	"errors"
	"io"

	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
)

func readExact(reader io.Reader) ([artifactregistry.EncodedSize]byte, error) {
	var encoded [artifactregistry.EncodedSize]byte
	if _, err := io.ReadFull(reader, encoded[:]); err != nil {
		return [artifactregistry.EncodedSize]byte{}, err
	}
	var extra [1]byte
	if n, err := io.ReadFull(reader, extra[:]); n != 0 || err != io.EOF {
		return [artifactregistry.EncodedSize]byte{}, errors.New("registry file has trailing bytes or cannot establish EOF")
	}
	return encoded, nil
}
