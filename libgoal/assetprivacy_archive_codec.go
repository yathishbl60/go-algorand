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

// encodeNativePrivacyScanPage encodes only a checkpoint and recovered openings.
// The caller encrypts this secret plaintext and clears it on every return path.
func encodeNativePrivacyScanPage(scanner *NativePrivacyScanner, page NativePrivacyScanPage) ([]byte, error) {
	if len(page.Notes) > 64 || checkNativePrivacyArchiveCheckpoint(scanner, page.Checkpoint, false) != nil {
		return nil, ErrNativePrivacyArchive
	}
	size := nativePrivacyArchivePlainSize
	plain := appendNativePrivacyArchiveCheckpoint(make([]byte, 0, size), page.Checkpoint)
	plain = append(plain, byte(len(page.Notes)))
	if err := appendNativePrivacyArchiveNotes(&plain, page); err != nil {
		clear(plain)
		return nil, ErrNativePrivacyArchive
	}
	return plain[:nativePrivacyArchivePlainSize], nil
}

func decodeNativePrivacyScanPage(scanner *NativePrivacyScanner, plain []byte) (NativePrivacyScanPage, error) {
	if len(plain) != nativePrivacyArchivePlainSize || plain[nativePrivacyArchiveCheckpointSize] > 64 {
		return NativePrivacyScanPage{}, ErrNativePrivacyArchive
	}
	end := nativePrivacyArchivePlainHeaderSize + int(plain[nativePrivacyArchiveCheckpointSize])*nativePrivacyArchiveRecordSize
	if !nativePrivacyArchiveZeroPadding(plain[end:]) {
		return NativePrivacyScanPage{}, ErrNativePrivacyArchive
	}
	checkpoint, err := readNativePrivacyArchiveCheckpoint(plain[:nativePrivacyArchiveCheckpointSize])
	if err != nil || checkNativePrivacyArchiveCheckpoint(scanner, checkpoint, true) != nil {
		return NativePrivacyScanPage{}, ErrNativePrivacyArchive
	}
	page := NativePrivacyScanPage{Checkpoint: checkpoint}
	if err = readNativePrivacyArchiveNotes(&page, plain[nativePrivacyArchivePlainHeaderSize:end]); err != nil {
		page.destroyScanNotes()
		return NativePrivacyScanPage{}, ErrNativePrivacyArchive
	}
	return page, nil
}
