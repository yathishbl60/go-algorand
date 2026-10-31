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
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

func (cs *roundCowState) assetPrivacyNeedsPQ(tx transactions.Transaction) (bool, error) {
	if tx.Type == protocol.NativeAssetPrivacyTx || tx.IsNativeAssetPrivacyCreation() {
		return true, nil
	}
	asset, admin := assetPrivacyAffectedAsset(tx)
	if asset == 0 {
		return false, nil
	}
	return cs.assetPrivacyExistingNeedsPQ(tx, asset, admin)
}

func assetPrivacyAffectedAsset(tx transactions.Transaction) (basics.AssetIndex, bool) {
	switch tx.Type {
	case protocol.AssetConfigTx:
		return tx.ConfigAsset, true
	case protocol.AssetFreezeTx:
		return tx.FreezeAsset, true
	case protocol.AssetTransferTx:
		return tx.XferAsset, !tx.AssetSender.IsZero()
	default:
		return 0, false
	}
}
