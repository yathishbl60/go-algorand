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

import "encoding/binary"

func (scanner *NativePrivacyScanner) scanArchiveAAD() ([]byte, error) {
	if scanner == nil || scanner.domain.Network == [32]byte{} || scanner.domain.Asset == 0 ||
		scanner.domain.Policy == [64]byte{} || scanner.domain.Disclosures > 3 {
		return nil, ErrNativePrivacyArchive
	}
	aad := append([]byte("Algorand/AssetPrivacy/ScanArchive/V1/"), nativePrivacyArchiveMagic...)
	aad = append(aad, scanner.domain.Network[:]...)
	aad = binary.BigEndian.AppendUint64(aad, uint64(scanner.domain.Asset))
	aad = append(aad, scanner.domain.Policy[:]...)
	return append(aad, byte(scanner.domain.Disclosures)), nil
}

// OpenNativePrivacyScanArchive restores a page offline without viewing keys or RPC.
// Its archived key-set remains unchanged; new viewing coverage requires rescanning.
func OpenNativePrivacyScanArchive(key NativePrivacyArchiveKey, domain NativePrivacyScanDomain, encoded []byte) (NativePrivacyScanPage, error) {
	return (&NativePrivacyScanner{domain: domain}).OpenScanPage(key, encoded)
}

func LoadNativePrivacyScanArchive(path string, key NativePrivacyArchiveKey, domain NativePrivacyScanDomain) (NativePrivacyScanPage, error) {
	return (&NativePrivacyScanner{domain: domain}).LoadScanPage(path, key)
}
