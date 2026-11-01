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
	"net/http"

	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/labstack/echo/v4"
)

func (v2 *Handlers) GetNativeAssetPrivacyKey(ctx echo.Context) error {
	value, err := v2.nativePrivacyKeyRequest(ctx)
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	response, err := nativePrivacyKeyResponse(value)
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	return ctx.JSON(http.StatusOK, response)
}

func (v2 *Handlers) nativePrivacyKeyRequest(ctx echo.Context) (query.KeySnapshot, error) {
	current, asset, err := v2.nativePrivacyAuthorizationRequest(ctx)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	account, err := nativePrivacyAccount(ctx)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	role, err := nativePrivacyRole(ctx)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	round, err := nativePrivacyRound(ctx, current)
	if err != nil {
		return query.KeySnapshot{}, err
	}
	return current.LookupNativeAssetPrivacyKey(round, asset, account, role)
}
