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
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
	"github.com/algorand/go-algorand/ledger"
	"github.com/algorand/go-algorand/ledger/ledgercore"
	"github.com/labstack/echo/v4"
	"net/http"
)

func (v2 *Handlers) nativePrivacyError(ctx echo.Context, err error) error {
	status, message := nativePrivacyErrorStatus(err)
	return returnError(ctx, status, err, message, v2.Log)
}

func nativePrivacyErrorStatus(err error) (int, string) {
	if errors.Is(err, errNativePrivacyParameters) || errors.Is(err, query.ErrCursor) || errors.Is(err, outputs.ErrPage) {
		return http.StatusBadRequest, "invalid native privacy request parameters or cursor"
	}
	if errors.Is(err, policy.ErrDisabled) || errors.Is(err, policy.ErrMissing) || errors.Is(err, query.ErrNotFound) || errors.Is(err, tree.ErrIndex) || errors.Is(err, outputs.ErrIndex) {
		return http.StatusNotFound, "native privacy asset or feature unavailable"
	}
	return nativePrivacyStateErrorStatus(err)
}

func nativePrivacyStateErrorStatus(err error) (int, string) {
	var oldRound *ledger.RoundOffsetError
	var noEntry ledgercore.ErrNoEntry
	if errors.Is(err, query.ErrUnavailable) || errors.As(err, &oldRound) || errors.As(err, &noEntry) {
		return http.StatusGone, "native privacy state or outputs no longer available at the requested round"
	}
	if nativePrivacyCorruptState(err) {
		return http.StatusInternalServerError, "inconsistent native privacy state"
	}
	return http.StatusServiceUnavailable, "native privacy ledger temporarily unavailable"
}
