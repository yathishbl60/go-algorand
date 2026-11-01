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

func appendNativePrivacyArchiveNotes(plain *[]byte, page NativePrivacyScanPage) error {
	for index, value := range page.Notes {
		if checkNativePrivacyArchiveNote(value, page.Checkpoint) != nil || index > 0 && page.Notes[index-1].index >= value.index {
			return ErrNativePrivacyArchive
		}
		encoded, err := value.opening.Encode()
		if err != nil {
			clear(encoded)
			return ErrNativePrivacyArchive
		}
		*plain = appendNativePrivacyArchiveNoteMetadata(*plain, value)
		*plain = append(*plain, encoded...)
		clear(encoded)
	}
	return nil
}

func appendNativePrivacyArchiveNoteMetadata(target []byte, value *NativePrivacyRecoveredNote) []byte {
	target = binary.BigEndian.AppendUint32(target, value.index)
	target = binary.BigEndian.AppendUint64(target, uint64(value.round))
	target = append(target, value.transaction[:]...)
	target = append(target, value.commitment[:]...)
	target = append(target, byte(value.role))
	return binary.BigEndian.AppendUint64(target, value.epoch)
}
