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
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/labstack/echo/v4"
)

type nativePrivacyAuthorizationLedger interface {
	nativePrivacyLedger
	LookupNativeAssetPrivacyAuthorization(basics.Round, basics.AssetIndex) (query.AuthorizationSnapshot, error)
	LookupNativeAssetPrivacyKey(basics.Round, basics.AssetIndex, basics.Address, keys.Role) (query.KeySnapshot, error)
	LookupNativeAssetPrivacyEligibility(basics.Round, basics.AssetIndex, basics.Address) (query.EligibilitySnapshot, error)
}

func (v2 *Handlers) nativePrivacyAuthorizationRequest(ctx echo.Context) (nativePrivacyAuthorizationLedger, basics.AssetIndex, error) {
	current, asset, err := v2.nativePrivacyRequest(ctx, "round")
	if err != nil {
		return nil, 0, err
	}
	reader, ok := current.(nativePrivacyAuthorizationLedger)
	if !ok {
		return nil, 0, errNativePrivacyLedger
	}
	return reader, asset, nil
}

func nativePrivacyAccount(ctx echo.Context) (basics.Address, error) {
	account, err := basics.UnmarshalChecksumAddress(ctx.Param("address"))
	if err != nil || account.IsZero() {
		return basics.Address{}, errNativePrivacyParameters
	}
	return account, nil
}

func nativePrivacyRole(ctx echo.Context) (keys.Role, error) {
	role, err := nativePrivacyNumber(ctx.Param("role"), uint64(keys.ManagerAuthority))
	if err != nil || role < uint64(keys.RecipientView) {
		return 0, errNativePrivacyParameters
	}
	return keys.Role(role), nil
}
