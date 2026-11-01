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
	"github.com/labstack/echo/v4"
	"net/http"
)

func (v2 *Handlers) GetNativeAssetPrivacyWitness(ctx echo.Context) error {
	current, asset, err := v2.nativePrivacyRequest(ctx, "round")
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	index, err := nativePrivacyNumber(ctx.Param("note-index"), 1<<32-1)
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	round, err := nativePrivacyRound(ctx, current)
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	witness, err := current.LookupNativeAssetPrivacyWitness(round, asset, uint32(index))
	if err != nil {
		return v2.nativePrivacyError(ctx, err)
	}
	return ctx.JSON(http.StatusOK, nativePrivacyWitnessResponse(witness))
}
