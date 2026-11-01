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

package libgoal

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"slices"
)

func GenerateNativePrivacyArchiveKey() (NativePrivacyArchiveKey, error) {
	var key NativePrivacyArchiveKey
	if _, err := rand.Read(key.material[:]); err != nil {
		return NativePrivacyArchiveKey{}, err
	}
	if key.material == [32]byte{} {
		return NativePrivacyArchiveKey{}, ErrNativePrivacyArchiveKey
	}
	return key, nil
}

func RestoreNativePrivacyArchiveKey(encoded []byte) (NativePrivacyArchiveKey, error) {
	if len(encoded) != 32 {
		return NativePrivacyArchiveKey{}, ErrNativePrivacyArchiveKey
	}
	var key NativePrivacyArchiveKey
	copy(key.material[:], encoded)
	if key.material == [32]byte{} {
		return NativePrivacyArchiveKey{}, ErrNativePrivacyArchiveKey
	}
	return key, nil
}

// Export returns owned secret bytes for a separate, protected key backup.
func (key NativePrivacyArchiveKey) Export() []byte { return slices.Clone(key.material[:]) }

func (key NativePrivacyArchiveKey) archiveCipher() (cipher.AEAD, error) {
	if key.material == [32]byte{} {
		return nil, ErrNativePrivacyArchiveKey
	}
	block, err := aes.NewCipher(key.material[:])
	if err != nil {
		return nil, ErrNativePrivacyArchiveKey
	}
	return cipher.NewGCMWithRandomNonce(block)
}
