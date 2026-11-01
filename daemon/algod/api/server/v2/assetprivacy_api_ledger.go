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
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/labstack/echo/v4"
)

var errNativePrivacyLedger = errors.New("native privacy ledger queries unavailable")

type nativePrivacyLedger interface {
	Latest() basics.Round
	LookupNativeAssetPrivacy(basics.Round, basics.AssetIndex) (query.Snapshot, error)
	LookupNativeAssetPrivacyOutputs(basics.Round, basics.AssetIndex, uint64, uint32) (query.OutputPage, error)
	LookupNativeAssetPrivacyOutputCursor(string, uint32) (query.OutputPage, error)
	LookupNativeAssetPrivacyWitness(basics.Round, basics.AssetIndex, uint32) (query.NoteWitness, error)
}

func (v2 *Handlers) nativePrivacyRequest(ctx echo.Context, allowed ...string) (nativePrivacyLedger, basics.AssetIndex, error) {
	if err := nativePrivacyQueryFields(ctx, allowed); err != nil {
		return nil, 0, err
	}
	asset, err := nativePrivacyNumber(ctx.Param("asset-id"), ^uint64(0))
	if err != nil || asset == 0 {
		return nil, 0, errNativePrivacyParameters
	}
	current, ok := v2.Node.LedgerForAPI().(nativePrivacyLedger)
	if !ok {
		return nil, 0, errNativePrivacyLedger
	}
	return current, basics.AssetIndex(asset), nil
}
