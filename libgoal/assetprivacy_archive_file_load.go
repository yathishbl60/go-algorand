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
	"io"
	"os"
)

// LoadScanPage reads a bounded regular archive, then authenticates it before any
// plaintext decoding. The path and its existing parent are caller-controlled.
func (scanner *NativePrivacyScanner) LoadScanPage(path string, key NativePrivacyArchiveKey) (NativePrivacyScanPage, error) {
	encoded, err := readNativePrivacyArchiveFile(path)
	if err != nil {
		return NativePrivacyScanPage{}, err
	}
	return scanner.OpenScanPage(key, encoded)
}

func readNativePrivacyArchiveFile(path string) ([]byte, error) {
	file, before, err := openNativePrivacyArchiveFile(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	encoded, err := io.ReadAll(io.LimitReader(file, int64(NativePrivacyScanArchiveMaxSize)+1))
	if err != nil {
		return nil, err
	}
	after, err := file.Stat()
	if err != nil {
		return nil, err
	}
	if len(encoded) != int(before.Size()) || !nativePrivacyArchiveFileShape(after) || after.Size() != before.Size() {
		return nil, ErrNativePrivacyArchive
	}
	return encoded, nil
}

func openNativePrivacyArchiveFile(path string) (*os.File, os.FileInfo, error) {
	before, err := os.Stat(path)
	if err != nil {
		return nil, nil, err
	}
	if !nativePrivacyArchiveFileShape(before) {
		return nil, nil, ErrNativePrivacyArchive
	}
	file, err := os.Open(path)
	if err != nil {
		return nil, nil, err
	}
	current, err := file.Stat()
	return checkNativePrivacyArchiveFile(file, before, current, err)
}

func checkNativePrivacyArchiveFile(file *os.File, before, current os.FileInfo, err error) (*os.File, os.FileInfo, error) {
	if err == nil && (!nativePrivacyArchiveFileShape(current) || !os.SameFile(before, current) || before.Size() != current.Size()) {
		err = ErrNativePrivacyArchive
	}
	if err != nil {
		file.Close()
		return nil, nil, err
	}
	return file, current, nil
}

func nativePrivacyArchiveFileShape(info os.FileInfo) bool {
	return info != nil && info.Mode().IsRegular() && info.Size() >= int64(nativePrivacyArchiveMinSize) &&
		info.Size() <= int64(NativePrivacyScanArchiveMaxSize)
}
