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

package inventory

import (
	"crypto/sha3"
	"encoding/binary"
	"github.com/algorand/go-algorand/internal/assetprivacy/storage"
)

func MetadataKey(asset uint64) (string, error) {
	if asset == 0 {
		return "", ErrIndex
	}
	var number [8]byte
	binary.BigEndian.PutUint64(number[:], asset)
	return prefix + string(number[:]) + "m", nil
}

func nodeKey(asset uint64, id ID) string {
	key, _ := MetadataKey(asset)
	return key[:len(key)-1] + "e" + string(id[:])
}

func identify(key string) ID {
	hash := sha3.New512()
	hash.Write([]byte("Algorand/AssetPrivacy/Candidate1/InventoryKey/"))
	var size [2]byte
	binary.BigEndian.PutUint16(size[:], uint16(len(key)))
	hash.Write(size[:])
	hash.Write([]byte(key))
	return ID(hash.Sum(nil))
}

func validateKey(asset uint64, key string) error {
	if len(key) == 0 || len(key) > MaxKeyBytes {
		return ErrChange
	}
	actual, native, err := storage.Asset(key)
	if err != nil || !native || actual != asset {
		return ErrChange
	}
	return nil
}
