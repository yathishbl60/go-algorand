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
	"github.com/algorand/go-algorand/data/basics"
	"github.com/labstack/echo/v4"
	"strconv"
)

var errNativePrivacyParameters = errors.New("invalid native privacy request parameters")

func nativePrivacyQueryFields(ctx echo.Context, allowed []string) error {
	for key, values := range ctx.QueryParams() {
		found := false
		for _, name := range allowed {
			found = found || key == name
		}
		if !found || len(values) != 1 {
			return errNativePrivacyParameters
		}
	}
	return nil
}

func nativePrivacyNumber(raw string, maximum uint64) (uint64, error) {
	number, err := strconv.ParseUint(raw, 10, 64)
	if err != nil || number > maximum {
		return 0, errNativePrivacyParameters
	}
	return number, nil
}

func nativePrivacyQueryNumber(ctx echo.Context, key string, fallback, maximum uint64) (uint64, error) {
	values, present := ctx.QueryParams()[key]
	if !present {
		return fallback, nil
	}
	if len(values) != 1 {
		return 0, errNativePrivacyParameters
	}
	return nativePrivacyNumber(values[0], maximum)
}

func nativePrivacyRound(ctx echo.Context, current nativePrivacyLedger) (basics.Round, error) {
	latest := uint64(current.Latest())
	round, err := nativePrivacyQueryNumber(ctx, "round", latest, latest)
	return basics.Round(round), err
}
