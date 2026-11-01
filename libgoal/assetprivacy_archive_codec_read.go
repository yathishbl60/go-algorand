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
	"encoding/binary"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/note"
)

func readNativePrivacyArchiveNotes(page *NativePrivacyScanPage, encoded []byte) error {
	for len(encoded) != 0 {
		value, err := readNativePrivacyArchiveNote(encoded[:nativePrivacyArchiveRecordSize], page.Checkpoint)
		if err != nil {
			return ErrNativePrivacyArchive
		}
		if len(page.Notes) > 0 && page.Notes[len(page.Notes)-1].index >= value.index {
			value.Destroy()
			return ErrNativePrivacyArchive
		}
		page.Notes = append(page.Notes, value)
		encoded = encoded[nativePrivacyArchiveRecordSize:]
	}
	return nil
}

func readNativePrivacyArchiveNote(encoded []byte, checkpoint NativePrivacyScanCheckpoint) (*NativePrivacyRecoveredNote, error) {
	opening, err := note.Decode(encoded[117:])
	if err != nil {
		return nil, ErrNativePrivacyArchive
	}
	value := &NativePrivacyRecoveredNote{opening: opening, network: checkpoint.Network,
		index: binary.BigEndian.Uint32(encoded[:4]), round: basics.Round(binary.BigEndian.Uint64(encoded[4:12])),
		role: apiclient.NativePrivacyRole(encoded[108]), epoch: binary.BigEndian.Uint64(encoded[109:117])}
	copy(value.transaction[:], encoded[12:44])
	copy(value.commitment[:], encoded[44:108])
	if checkNativePrivacyArchiveNote(value, checkpoint) != nil {
		value.Destroy()
		return nil, ErrNativePrivacyArchive
	}
	return value, nil
}
