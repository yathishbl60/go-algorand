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

import apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"

// Old key coverage remains on restored pages, so ResumePage still rejects it.
func checkNativePrivacyArchiveCheckpoint(scanner *NativePrivacyScanner, checkpoint NativePrivacyScanCheckpoint, allowOldKeys bool) error {
	if _, err := scanner.scanArchiveAAD(); err != nil || checkpoint.KeySet == [64]byte{} {
		return ErrNativePrivacyArchive
	}
	if !allowOldKeys && (scanner.client == nil || len(scanner.keys) == 0) {
		return ErrNativePrivacyArchive
	}
	if allowOldKeys {
		checkpoint.KeySet = scanner.coverage
	}
	if scanner.checkScanCheckpoint(checkpoint) != nil {
		return ErrNativePrivacyArchive
	}
	return nil
}

func checkNativePrivacyArchiveNote(value *NativePrivacyRecoveredNote, checkpoint NativePrivacyScanCheckpoint) error {
	if value == nil || value.network != checkpoint.Network || value.opening.Asset != uint64(checkpoint.Asset) || value.opening.Policy != checkpoint.Policy ||
		value.round > checkpoint.Round || uint64(value.index) >= checkpoint.Next || uint64(value.index) >= checkpoint.Count || value.epoch == 0 ||
		!nativePrivacyArchiveRole(value.role, checkpoint.Disclosures) || value.opening.Validate() != nil {
		return ErrNativePrivacyArchive
	}
	commitment, err := value.opening.Commitment(value.network)
	if err != nil || commitment != value.commitment {
		return ErrNativePrivacyArchive
	}
	return nil
}

func nativePrivacyArchiveRole(role apiclient.NativePrivacyRole, disclosures uint64) bool {
	switch role {
	case apiclient.NativePrivacyRecipientView:
		return true
	case apiclient.NativePrivacyAuditorView:
		return disclosures&1 != 0
	case apiclient.NativePrivacyControllerView:
		return disclosures&2 != 0
	}
	return false
}

// Padding is canonical and hides the number of matches in the encrypted length.
func nativePrivacyArchiveZeroPadding(encoded []byte) bool {
	for _, value := range encoded {
		if value != 0 {
			return false
		}
	}
	return true
}
