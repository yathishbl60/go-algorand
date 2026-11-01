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

// SealScanPage encrypts notes and progress together. A new nonce is always generated.
// Keep the page stable during sealing. No plaintext is written or sent to RPC.
func (scanner *NativePrivacyScanner) SealScanPage(key NativePrivacyArchiveKey, page NativePrivacyScanPage) ([]byte, error) {
	aad, err := scanner.scanArchiveAAD()
	if err != nil {
		return nil, err
	}
	plain, err := encodeNativePrivacyScanPage(scanner, page)
	if err != nil {
		return nil, err
	}
	defer clear(plain)
	return sealNativePrivacyScanArchive(key, plain, aad)
}

func (scanner *NativePrivacyScanner) OpenScanPage(key NativePrivacyArchiveKey, encoded []byte) (NativePrivacyScanPage, error) {
	aad, err := scanner.scanArchiveAAD()
	if err != nil {
		return NativePrivacyScanPage{}, err
	}
	plain, err := openNativePrivacyScanArchive(key, encoded, aad)
	if err != nil {
		return NativePrivacyScanPage{}, err
	}
	defer clear(plain)
	return decodeNativePrivacyScanPage(scanner, plain)
}

func sealNativePrivacyScanArchive(key NativePrivacyArchiveKey, plain, aad []byte) ([]byte, error) {
	aead, err := key.archiveCipher()
	if err != nil {
		return nil, err
	}
	if len(plain) != nativePrivacyArchivePlainSize {
		return nil, ErrNativePrivacyArchive
	}
	return aead.Seal([]byte(nativePrivacyArchiveMagic), nil, plain, aad), nil
}

func openNativePrivacyScanArchive(key NativePrivacyArchiveKey, encoded, aad []byte) ([]byte, error) {
	if len(encoded) != NativePrivacyScanArchiveMaxSize || string(encoded[:len(nativePrivacyArchiveMagic)]) != nativePrivacyArchiveMagic {
		return nil, ErrNativePrivacyArchive
	}
	aead, err := key.archiveCipher()
	if err != nil {
		return nil, err
	}
	plain, err := aead.Open(nil, nil, encoded[len(nativePrivacyArchiveMagic):], aad)
	if err != nil {
		return nil, ErrNativePrivacyArchive
	}
	return plain, nil
}
