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

package simulation

import (
	"fmt"

	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/protocol"
)

// Native account consent must remain real even when other simulation signatures are optional.
func checkNativePrivacySimulationSigners(group []transactions.SignedTxnWithAD, overrides ResultEvalOverrides) error {
	if !overrides.AllowEmptySignatures && !overrides.FixSigners {
		return nil
	}
	for index, tx := range group {
		native := tx.Txn.Type == protocol.NativeAssetPrivacyTx || tx.Txn.IsNativeAssetPrivacyCreation()
		if native && txnNeedsSyntheticSignature(tx.SignedTxn) {
			err := fmt.Errorf("transaction %d: native privacy simulation requires real post-quantum authorization; a synthetic signature cannot replace it", index)
			return InvalidRequestError{SimulatorError{err}}
		}
	}
	return nil
}
