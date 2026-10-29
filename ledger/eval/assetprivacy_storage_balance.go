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

func (cs *roundCowState) assetPrivacyMinimumBalance(payer basics.Address, base basics.MicroAlgos) (basics.MicroAlgos, error) {
	extra, err := cs.assetPrivacyStorageMinimum(payer)
	if err != nil {
		return basics.MicroAlgos{}, err
	}
	value, overflow := basics.OAdd(base.Raw, extra)
	if overflow {
		return basics.MicroAlgos{}, storage.ErrArithmetic
	}
	return basics.MicroAlgos{Raw: value}, nil
}

func (cs *roundCowState) checkAssetPrivacyStorageBalance(payer basics.Address) error {
	extra, err := cs.assetPrivacyStorageMinimum(payer)
	if err != nil || extra == 0 {
		return err
	}
	data, err := cs.Get(payer, true)
	if err != nil {
		return err
	}
	minimum, err := cs.assetPrivacyMinimumBalance(payer, data.MinBalance(&cs.proto))
	if err != nil {
		return err
	}
	return cs.checkAssetPrivacyFunded(payer, data, minimum.Raw)
}

func (cs *roundCowState) checkAssetPrivacyFunded(payer basics.Address, data ledgercore.AccountData, minimum uint64) error {
	if data.MicroAlgos.Raw < minimum {
		return &ledgercore.MinBalanceError{Account: payer, Balance: data.MicroAlgos.Raw,
			MinBalance: minimum, TotalAssets: data.TotalAssets}
	}
	if cs.proto.MaximumMinimumBalance != 0 && minimum > cs.proto.MaximumMinimumBalance {
		return storage.ErrArithmetic
	}
	return nil
}
