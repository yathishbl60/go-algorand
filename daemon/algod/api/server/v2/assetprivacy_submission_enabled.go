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

	"github.com/algorand/go-algorand/data/transactions/verify"
)

func (v2 *Handlers) nativePrivacySubmissionError(ctx echo.Context, err error) error {
	if verify.IsLocalNativePrivacyFailure(err) {
		return serviceUnavailable(ctx, err, "native privacy verification temporarily unavailable", v2.Log)
	}
	return badRequest(ctx, err, err.Error(), v2.Log)
}
