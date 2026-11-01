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

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
)

// ScanPage is one bounded, trusted-RPC recovery pass, with no durable mutation.
// A new or expanded historical key set requires rescanning old output ranges.
func (scanner *NativePrivacyScanner) ScanPage(ctx context.Context, options apiclient.NativePrivacyOutputOptions) (NativePrivacyScanPage, error) {
	if err := scanner.scanContext(ctx); err != nil {
		return NativePrivacyScanPage{}, err
	}
	page, err := scanner.publicScanPage(ctx, options)
	if err != nil {
		return NativePrivacyScanPage{}, err
	}
	return scanner.recoverScanPage(ctx, page)
}

func (scanner *NativePrivacyScanner) ResumePage(ctx context.Context, checkpoint NativePrivacyScanCheckpoint, limit uint32) (NativePrivacyScanPage, error) {
	if err := scanner.scanContext(ctx); err != nil {
		return NativePrivacyScanPage{}, err
	}
	if err := scanner.checkScanCheckpoint(checkpoint); err != nil {
		return NativePrivacyScanPage{}, err
	}
	if checkpoint.Next == checkpoint.Count {
		return NativePrivacyScanPage{}, ErrNativePrivacyScanComplete
	}
	return scanner.ScanPage(ctx, apiclient.NativePrivacyOutputOptions{Next: *checkpoint.NextToken, Limit: limit})
}

func (scanner *NativePrivacyScanner) scanContext(ctx context.Context) error {
	if scanner == nil || scanner.client == nil || ctx == nil {
		return ErrNativePrivacyScanConfig
	}
	return ctx.Err()
}
