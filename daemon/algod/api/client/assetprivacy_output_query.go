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
	"net/url"
	"strconv"

	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

type nativePrivacyExpectedPage struct {
	start  uint64
	limit  uint32
	cursor *query.Cursor
}

func nativePrivacyOutputQuery(asset basics.AssetIndex, options NativePrivacyOutputOptions) (url.Values, nativePrivacyExpectedPage, error) {
	limit := options.Limit
	if limit == 0 {
		limit = 32
	}
	if limit > 64 || options.Start > 1<<32 {
		return nil, nativePrivacyExpectedPage{}, ErrNativePrivacyRequest
	}
	expected := nativePrivacyExpectedPage{start: options.Start, limit: limit}
	if options.Next != "" {
		return nativePrivacyOutputCursor(asset, options, expected)
	}
	values := nativePrivacyRoundQuery(options.Round)
	values.Set("start", strconv.FormatUint(options.Start, 10))
	values.Set("limit", strconv.FormatUint(uint64(limit), 10))
	return values, expected, nil
}

func nativePrivacyOutputCursor(asset basics.AssetIndex, options NativePrivacyOutputOptions, expected nativePrivacyExpectedPage) (url.Values, nativePrivacyExpectedPage, error) {
	if options.Round != nil || options.Start != 0 {
		return nil, expected, ErrNativePrivacyRequest
	}
	cursor, err := query.DecodeCursor(options.Next)
	if err != nil || cursor.Asset != uint64(asset) {
		return nil, expected, ErrNativePrivacyRequest
	}
	expected.start, expected.cursor = cursor.Next, &cursor
	values := make(url.Values)
	values.Set("next", options.Next)
	values.Set("limit", strconv.FormatUint(uint64(expected.limit), 10))
	return values, expected, nil
}
