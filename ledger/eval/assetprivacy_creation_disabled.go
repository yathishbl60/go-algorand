//go:build !algorand_native_privacy

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
	"errors"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/data/transactions/logic"
	"github.com/algorand/go-algorand/ledger/apply"
)

func (eval *BlockEvaluator) applyAssetConfigWithPrivacy(tx transactions.Transaction, cs *roundCowState, _ *logic.EvalParams, _ int, ad *transactions.ApplyData, counter uint64) error {
	if len(tx.NativeAssetPrivacy) != 0 {
		return errors.New("native asset privacy creation requires the enabled build")
	}
	return apply.AssetConfig(tx.AssetConfigTxnFields, tx.Header, cs, eval.specials, ad, counter)
}
