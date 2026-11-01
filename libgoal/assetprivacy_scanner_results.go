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
	"context"

	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
)

func (scanner *NativePrivacyScanner) recoverScanPage(ctx context.Context, page model.NativeAssetPrivacyOutputPage) (NativePrivacyScanPage, error) {
	result := NativePrivacyScanPage{Checkpoint: scanner.scanCheckpoint(page)}
	if err := scanner.checkScanCheckpoint(result.Checkpoint); err != nil {
		return NativePrivacyScanPage{}, err
	}
	if err := scanner.recoverScanRecords(ctx, page.Outputs, &result); err != nil {
		result.destroyScanNotes()
		return NativePrivacyScanPage{}, err
	}
	if err := ctx.Err(); err != nil {
		result.destroyScanNotes()
		return NativePrivacyScanPage{}, err
	}
	return result, nil
}

func (scanner *NativePrivacyScanner) recoverScanRecords(ctx context.Context, records []model.NativeAssetPrivacyOutput, result *NativePrivacyScanPage) error {
	for _, record := range records {
		if err := ctx.Err(); err != nil {
			return err
		}
		match, err := scanner.recoverScanOutput(ctx, record)
		if err != nil {
			return err
		}
		if match != nil {
			result.Notes = append(result.Notes, match)
		}
	}
	return nil
}

func (result *NativePrivacyScanPage) destroyScanNotes() {
	for _, opening := range result.Notes {
		opening.Destroy()
	}
	clear(result.Notes)
	result.Notes = nil
}
