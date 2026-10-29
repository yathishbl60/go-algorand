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
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
)

func assetPrivacyExecutionGroup(group []transactions.SignedTxnWithAD, proto config.ConsensusParams) ([]transactions.Transaction, error) {
	if proto.MaxTxnBytesPerBlock <= 0 || len(group) == 0 || len(group) > proto.MaxTxGroupSize {
		return nil, statement.ErrLimit
	}
	remaining := proto.MaxTxnBytesPerBlock
	bodies := make([]transactions.Transaction, len(group))
	for i := range group {
		size := group[i].SignedTxn.Msgsize()
		if size <= 0 || size > remaining {
			return nil, statement.ErrLimit
		}
		remaining -= size
		bodies[i] = group[i].Txn
	}
	return bodies, nil
}

func (eval *BlockEvaluator) checkAssetPrivacyExecutionFees(group []transactions.SignedTxnWithAD) error {
	usage, paid := transactions.SummarizeFees(group, eval.proto)
	return CheckGroupFees(paid, usage, eval.proto.MinFee())
}
