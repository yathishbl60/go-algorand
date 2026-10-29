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
	"encoding/hex"
	"fmt"
	"path/filepath"

	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
)

func (d *directory) Get(expected [64]byte) ([artifactregistry.EncodedSize]byte, error) {
	path, err := d.filename(expected)
	if err != nil {
		return [artifactregistry.EncodedSize]byte{}, err
	}
	file, err := openRegular(path)
	if err != nil {
		return [artifactregistry.EncodedSize]byte{}, unavailable(err)
	}
	defer file.Close()
	encoded, err := readExact(file)
	if err != nil {
		return [artifactregistry.EncodedSize]byte{}, unavailable(err)
	}
	return checkDigest(encoded, expected)
}

func (d *directory) filename(expected [64]byte) (string, error) {
	if d == nil || d.root == "" || expected == [64]byte{} {
		return "", fmt.Errorf("%w: missing directory or registry digest", ErrUnavailable)
	}
	return filepath.Join(d.root, hex.EncodeToString(expected[:])+".napk"), nil
}

func checkDigest(encoded [artifactregistry.EncodedSize]byte, expected [64]byte) ([artifactregistry.EncodedSize]byte, error) {
	actual, err := artifactregistry.Digest(encoded[:])
	if err != nil || actual != expected {
		return [artifactregistry.EncodedSize]byte{}, fmt.Errorf("%w: registry digest mismatch", ErrUnavailable)
	}
	return encoded, nil
}

func unavailable(err error) error { return fmt.Errorf("%w: %v", ErrUnavailable, err) }
