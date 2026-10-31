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

package eval

import (
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

// Reject unavailable or frozen public funds during native planning, before proof
// work. Application repeats the checks against the same execution child.
func (cs *roundCowState) checkAssetPrivacyDebitState(body nativebody.Body) error {
	if body.Debit == 0 {
		return nil
	}
	holding, err := cs.assetPrivacyPublicHolding(basics.AssetIndex(body.Asset), basics.Address(body.DebitSource), false)
	if err != nil {
		return err
	}
	if holding.Amount < body.Debit {
		return &ledgercore.AssetBalanceError{Amount: body.Debit, SenderAmount: holding.Amount}
	}
	return nil
}
