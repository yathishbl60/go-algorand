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

// RekeyNativePrivacyScanArchive replaces only archive encryption, preserving
// historical viewing coverage. It performs no RPC or spending authorization.
func RekeyNativePrivacyScanArchive(oldKey, newKey NativePrivacyArchiveKey, domain NativePrivacyScanDomain, encoded []byte) ([]byte, error) {
	scanner := &NativePrivacyScanner{domain: domain}
	aad, err := scanner.scanArchiveAAD()
	if err != nil {
		return nil, err
	}
	plain, err := openNativePrivacyScanArchive(oldKey, encoded, aad)
	if err != nil {
		return nil, err
	}
	defer clear(plain)
	page, err := decodeNativePrivacyScanPage(scanner, plain)
	if err != nil {
		return nil, err
	}
	defer page.destroyScanNotes()
	return sealNativePrivacyScanArchive(newKey, plain, aad)
}

// RekeyNativePrivacyScanArchiveFile preserves the old file and publishes a new
// immutable encrypted copy. A successful rekey does not revoke earlier copies.
func RekeyNativePrivacyScanArchiveFile(oldPath, newPath string, oldKey, newKey NativePrivacyArchiveKey, domain NativePrivacyScanDomain) error {
	encoded, err := readNativePrivacyArchiveFile(oldPath)
	if err != nil {
		return err
	}
	rekeyed, err := RekeyNativePrivacyScanArchive(oldKey, newKey, domain, encoded)
	if err != nil {
		return err
	}
	return saveNativePrivacyArchiveFile(newPath, rekeyed)
}
