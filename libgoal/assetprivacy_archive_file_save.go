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
	"fmt"
	"io"
	"os"
	"path/filepath"
)

// SaveScanPage seals a page before touching the filesystem and publishes one
// immutable archive. Existing entries, including symlinks, are never replaced.
func (scanner *NativePrivacyScanner) SaveScanPage(path string, key NativePrivacyArchiveKey, page NativePrivacyScanPage) error {
	encoded, err := scanner.SealScanPage(key, page)
	if err != nil {
		return err
	}
	return saveNativePrivacyArchiveFile(path, encoded)
}

func saveNativePrivacyArchiveFile(path string, encoded []byte) error {
	parent := filepath.Dir(path)
	file, err := os.CreateTemp(parent, ".native-privacy-scan-*")
	if err != nil {
		return err
	}
	defer cleanupNativePrivacyArchiveFile(file)
	if err = writeNativePrivacyArchiveFile(file, encoded); err != nil {
		return err
	}
	if err = os.Link(file.Name(), path); err != nil {
		return err
	}
	return syncNativePrivacyArchiveDirectory(parent)
}

func writeNativePrivacyArchiveFile(file *os.File, encoded []byte) error {
	written, err := file.Write(encoded)
	if err != nil {
		return err
	}
	if written != len(encoded) {
		return io.ErrShortWrite
	}
	if err = file.Sync(); err != nil {
		return err
	}
	return file.Close()
}

func syncNativePrivacyArchiveDirectory(parent string) error {
	directory, err := os.Open(parent)
	if err == nil {
		defer directory.Close()
		err = directory.Sync()
	}
	if err != nil {
		return fmt.Errorf("%w: %w", ErrNativePrivacyArchiveDurability, err)
	}
	return nil
}

func cleanupNativePrivacyArchiveFile(file *os.File) {
	file.Close()
	os.Remove(file.Name())
}
