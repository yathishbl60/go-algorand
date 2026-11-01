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
	"bytes"

	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (scanner *NativePrivacyScanner) scanCheckpoint(page model.NativeAssetPrivacyOutputPage) NativePrivacyScanCheckpoint {
	checkpoint := NativePrivacyScanCheckpoint{NativePrivacyScanDomain: scanner.domain, Version: 1,
		KeySet: scanner.coverage, Round: basics.Round(page.Round), Count: page.Count, Next: page.Next}
	copy(checkpoint.Root[:], page.Root)
	if page.NextToken != nil {
		token := *page.NextToken
		checkpoint.NextToken = &token
	}
	return checkpoint
}

func (scanner *NativePrivacyScanner) checkScanCheckpoint(checkpoint NativePrivacyScanCheckpoint) error {
	if checkpoint.Version != 1 || checkpoint.NativePrivacyScanDomain != scanner.domain || checkpoint.KeySet != scanner.coverage ||
		checkpoint.Count > 1<<32 || checkpoint.Next > checkpoint.Count || checkpoint.Root == [64]byte{} {
		return ErrNativePrivacyScanCheckpoint
	}
	if checkpoint.Next == checkpoint.Count {
		if checkpoint.NextToken != nil {
			return ErrNativePrivacyScanCheckpoint
		}
		return nil
	}
	return checkNativePrivacyScanCursor(checkpoint)
}

func checkNativePrivacyScanCursor(checkpoint NativePrivacyScanCheckpoint) error {
	if checkpoint.NextToken == nil {
		return ErrNativePrivacyScanCheckpoint
	}
	cursor, err := query.DecodeCursor(*checkpoint.NextToken)
	if err != nil || cursor.Asset != uint64(checkpoint.Asset) || cursor.Round != uint64(checkpoint.Round) ||
		cursor.Count != checkpoint.Count || cursor.Next != checkpoint.Next ||
		!bytes.Equal(cursor.PolicyID[:], checkpoint.Policy[:]) || !bytes.Equal(cursor.Root[:], checkpoint.Root[:]) {
		return ErrNativePrivacyScanCheckpoint
	}
	return nil
}
