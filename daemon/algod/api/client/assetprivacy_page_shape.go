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

package client

import (
	"bytes"

	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (expected nativePrivacyExpectedPage) check(value model.NativeAssetPrivacyOutputPage) error {
	if len(value.PolicyId) != 64 || len(value.Root) != 64 || value.Count > 1<<32 || value.Count < expected.start {
		return ErrNativePrivacyResponse
	}
	end := min(value.Count, expected.start+uint64(expected.limit))
	if value.Next != end || len(value.Outputs) != int(end-expected.start) {
		return ErrNativePrivacyResponse
	}
	if expected.cursor != nil {
		cursor := expected.cursor
		if cursor.Round != value.Round || cursor.Count != value.Count || !bytes.Equal(cursor.PolicyID[:], value.PolicyId) || !bytes.Equal(cursor.Root[:], value.Root) {
			return ErrNativePrivacyResponse
		}
	}
	return expected.records(value)
}

func (expected nativePrivacyExpectedPage) records(value model.NativeAssetPrivacyOutputPage) error {
	for i, record := range value.Outputs {
		if record.Index != expected.start+uint64(i) || record.Round > value.Round || !nativePrivacyOutputShape(record) {
			return ErrNativePrivacyResponse
		}
	}
	return nativePrivacyNextShape(value)
}

func nativePrivacyNextShape(value model.NativeAssetPrivacyOutputPage) error {
	if value.Next == value.Count {
		if value.NextToken != nil {
			return ErrNativePrivacyResponse
		}
		return nil
	}
	if value.NextToken == nil {
		return ErrNativePrivacyResponse
	}
	next, err := query.DecodeCursor(*value.NextToken)
	if err != nil || next.Round != value.Round || next.Asset != value.AssetId || next.Count != value.Count || next.Next != value.Next ||
		!bytes.Equal(next.PolicyID[:], value.PolicyId) || !bytes.Equal(next.Root[:], value.Root) {
		return ErrNativePrivacyResponse
	}
	return nil
}
