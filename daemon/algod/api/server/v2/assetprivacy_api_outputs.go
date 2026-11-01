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

package v2

import (
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/labstack/echo/v4"
	"net/http"
)

func (v2 *Handlers) GetNativeAssetPrivacyOutputs(ctx echo.Context) error {
	current, asset, err := v2.nativePrivacyRequest(ctx, "round", "start", "limit", "next")
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	page, err := nativePrivacyOutputRequest(ctx, current, asset)
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	response, err := nativePrivacyPageResponse(page)
	if err != nil {
		return v2.nativePrivacyError(ctx, query.ErrState)
	}
	return ctx.JSON(http.StatusOK, response)
}

func nativePrivacyOutputRequest(ctx echo.Context, current nativePrivacyLedger, asset basics.AssetIndex) (query.OutputPage, error) {
	limit, err := nativePrivacyQueryNumber(ctx, "limit", 32, 64)
	if err != nil || limit == 0 {
		return query.OutputPage{}, errNativePrivacyParameters
	}
	if next, present := ctx.QueryParams()["next"]; present {
		return nativePrivacyCursorRequest(ctx, current, asset, next[0], uint32(limit))
	}
	round, err := nativePrivacyRound(ctx, current)
	if err != nil {
		return query.OutputPage{}, err
	}
	start, err := nativePrivacyQueryNumber(ctx, "start", 0, 1<<32)
	if err != nil {
		return query.OutputPage{}, err
	}
	return current.LookupNativeAssetPrivacyOutputs(round, asset, start, uint32(limit))
}

func nativePrivacyCursorRequest(ctx echo.Context, current nativePrivacyLedger, asset basics.AssetIndex, next string, limit uint32) (query.OutputPage, error) {
	for _, field := range []string{"round", "start"} {
		if _, present := ctx.QueryParams()[field]; present {
			return query.OutputPage{}, errNativePrivacyParameters
		}
	}
	cursor, err := query.DecodeCursor(next)
	if err != nil || cursor.Asset != uint64(asset) {
		return query.OutputPage{}, query.ErrCursor
	}
	if cursor.Round > uint64(current.Latest()) {
		return query.OutputPage{}, errNativePrivacyParameters
	}
	return current.LookupNativeAssetPrivacyOutputCursor(next, limit)
}
