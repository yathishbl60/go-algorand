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

import "github.com/algorand/go-algorand/data/basics"

// AssetPrivacyTransactionPlan derives only from a canonical target transaction.
// It establishes neither signed consent nor atomic execution authority.
type AssetPrivacyTransactionPlan struct {
	spend      *AssetPrivacySpendPlan
	recovery   *AssetPrivacyRecoveryPlan
	conversion AssetPrivacyPublicConversion
}

// AssetPrivacyPublicConversion contains the exact public account/amount body fields.
type AssetPrivacyPublicConversion struct {
	Asset          basics.AssetIndex
	Debit          uint64
	DebitSource    basics.Address
	Credit         uint64
	CreditReceiver basics.Address
}

func (p *AssetPrivacyTransactionPlan) Spend() *AssetPrivacySpendPlan {
	if p == nil {
		return nil
	}
	return copyAssetPrivacyPlan(p.spend)
}

func (p *AssetPrivacyTransactionPlan) Recovery() *AssetPrivacyRecoveryPlan {
	if p == nil {
		return nil
	}
	return copyAssetPrivacyPlan(p.recovery)
}

func (p *AssetPrivacyTransactionPlan) PublicConversion() AssetPrivacyPublicConversion {
	if p == nil {
		return AssetPrivacyPublicConversion{}
	}
	return p.conversion
}
