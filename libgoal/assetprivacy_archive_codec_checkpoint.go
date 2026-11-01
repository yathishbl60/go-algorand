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

	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func appendNativePrivacyArchiveCheckpoint(target []byte, checkpoint NativePrivacyScanCheckpoint) []byte {
	target = append(target, checkpoint.Version)
	target = append(target, checkpoint.Network[:]...)
	target = binary.BigEndian.AppendUint64(target, uint64(checkpoint.Asset))
	target = append(target, checkpoint.Policy[:]...)
	target = append(target, byte(checkpoint.Disclosures))
	target = append(target, checkpoint.KeySet[:]...)
	target = binary.BigEndian.AppendUint64(target, uint64(checkpoint.Round))
	target = append(target, checkpoint.Root[:]...)
	target = binary.BigEndian.AppendUint64(target, checkpoint.Count)
	return binary.BigEndian.AppendUint64(target, checkpoint.Next)
}

func readNativePrivacyArchiveCheckpoint(encoded []byte) (NativePrivacyScanCheckpoint, error) {
	checkpoint := NativePrivacyScanCheckpoint{Version: encoded[0],
		NativePrivacyScanDomain: NativePrivacyScanDomain{Asset: basics.AssetIndex(binary.BigEndian.Uint64(encoded[33:41])), Disclosures: uint64(encoded[105])},
		Round:                   basics.Round(binary.BigEndian.Uint64(encoded[170:178])),
		Count:                   binary.BigEndian.Uint64(encoded[242:250]), Next: binary.BigEndian.Uint64(encoded[250:258])}
	copy(checkpoint.Network[:], encoded[1:33])
	copy(checkpoint.Policy[:], encoded[41:105])
	copy(checkpoint.KeySet[:], encoded[106:170])
	copy(checkpoint.Root[:], encoded[178:242])
	if checkpoint.Next < checkpoint.Count {
		cursor := query.Cursor{Round: uint64(checkpoint.Round), Asset: uint64(checkpoint.Asset),
			PolicyID: checkpoint.Policy, Root: checkpoint.Root, Count: checkpoint.Count, Next: checkpoint.Next}
		token, err := cursor.Encode()
		if err != nil {
			return NativePrivacyScanCheckpoint{}, ErrNativePrivacyArchive
		}
		checkpoint.NextToken = &token
	}
	return checkpoint, nil
}
