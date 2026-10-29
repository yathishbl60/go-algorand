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
	"github.com/algorand/go-algorand/internal/assetprivacy/storage"
	"github.com/algorand/go-algorand/ledger/ledgercore"
)

func (cs *roundCowState) assetPrivacyFundingAccount(payer basics.Address, obligation storage.Obligation) (ledgercore.AccountData, error) {
	extra, err := obligation.Required(cs.proto.NativeAssetPrivacyFlatMinBalance, cs.proto.NativeAssetPrivacyByteMinBalance)
	if err != nil {
		return ledgercore.AccountData{}, err
	}
	data, err := cs.Get(payer, false)
	if err != nil {
		return ledgercore.AccountData{}, err
	}
	funded := data.WithUpdatedRewards(cs.proto.RewardUnit, cs.rewardsLevel())
	minimum, overflow := basics.OAdd(funded.MinBalance(&cs.proto).Raw, extra)
	if overflow {
		return ledgercore.AccountData{}, storage.ErrArithmetic
	}
	if err = cs.checkAssetPrivacyFunded(payer, funded, minimum); err != nil {
		return ledgercore.AccountData{}, err
	}
	return data, nil
}
