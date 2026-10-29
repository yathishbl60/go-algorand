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
	"fmt"

	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

func (cs *roundCowState) assetPrivacyPublicDebit(asset basics.AssetIndex, source basics.Address, amount uint64) error {
	if amount == 0 {
		return nil
	}
	holding, err := cs.assetPrivacyPublicHolding(asset, source, false)
	if err != nil {
		return err
	}
	next, overflow := basics.OSub(holding.Amount, amount)
	if overflow {
		return &ledgercore.AssetBalanceError{Amount: amount, SenderAmount: holding.Amount}
	}
	holding.Amount = next
	return cs.PutAssetHolding(source, asset, holding)
}

func (cs *roundCowState) assetPrivacyPublicCredit(asset basics.AssetIndex, receiver basics.Address, amount uint64, recovery bool) error {
	if amount == 0 {
		return nil
	}
	holding, err := cs.assetPrivacyPublicHolding(asset, receiver, recovery)
	if err != nil {
		return err
	}
	next, overflow := basics.OAdd(holding.Amount, amount)
	if overflow {
		return fmt.Errorf("asset %d public credit of %d overflows holding amount %d", asset, amount, holding.Amount)
	}
	holding.Amount = next
	return cs.PutAssetHolding(receiver, asset, holding)
}
