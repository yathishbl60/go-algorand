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

package pools

import (
	"github.com/algorand/go-algorand/data/bookkeeping"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/data/transactions/verify"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
	"github.com/algorand/go-algorand/logging/telemetryspec"
)

// Protected by pool.mu. Deferred groups remain in the existing bounded queue.
type nativePrivacyPoolState struct{ deferred bool }

func (pool *TransactionPool) resetNativePrivacyDeferral() {
	pool.nativePrivacyState.deferred = false
}

// Fresh admission cannot evaluate against an unresolved pending prefix.
func (pool *TransactionPool) nativePrivacyAdmissionError() error {
	if pool.nativePrivacyState.deferred {
		return proofbridge.ErrResource
	}
	return nil
}

func (pool *TransactionPool) readdNativePrivacyGroup(group []transactions.SignedTxn, stats *telemetryspec.AssembleBlockMetrics) (error, bool) {
	if pool.nativePrivacyState.deferred {
		return pool.retainNativePrivacyGroup(group)
	}
	err := pool.add(group, stats)
	if !verify.IsLocalNativePrivacyFailure(err) {
		return err, false
	}
	pool.nativePrivacyState.deferred = true
	return pool.retainNativePrivacyGroup(group)
}

// Retention is not evaluator acceptance. Retry in order on the next recompute.
func (pool *TransactionPool) retainNativePrivacyGroup(group []transactions.SignedTxn) (error, bool) {
	round := pool.pendingBlockEvaluator.Round() + pool.numPendingWholeBlocks
	for _, tx := range group {
		if tx.Txn.LastValid < round {
			return &bookkeeping.TxnDeadError{Round: round, FirstValid: tx.Txn.FirstValid,
				LastValid: tx.Txn.LastValid, Early: false}, false
		}
	}
	pool.rememberedTxGroups = append(pool.rememberedTxGroups, group)
	for _, tx := range group {
		pool.rememberedTxids[tx.ID()] = tx
	}
	return nil, true
}
