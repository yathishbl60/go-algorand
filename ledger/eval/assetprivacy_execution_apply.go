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
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/data/transactions/logic"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func (eval *BlockEvaluator) applyNativeAssetPrivacy(cow *roundCowState, params *logic.EvalParams, gi int) error {
	if !cow.proto.NativeAssetPrivacyExecutionConfigured() {
		return policy.ErrDisabled
	}
	if params == nil || params.GetCaller() != nil || gi < 0 || gi >= len(params.TxnGroup) {
		return policy.ErrDisabled
	}
	child := cow.child(0)
	defer child.recycle()
	if err := eval.executeNativeAssetPrivacy(child, params, gi); err != nil {
		return err
	}
	child.commitToParent()
	return nil
}

func (eval *BlockEvaluator) executeNativeAssetPrivacy(cs *roundCowState, params *logic.EvalParams, gi int) error {
	group, err := assetPrivacyExecutionGroup(params.TxnGroup, cs.proto)
	if err != nil {
		return err
	}
	if err = eval.checkAssetPrivacyExecutionFees(params.TxnGroup); err != nil {
		return err
	}
	if lifecyclebody.IsLifecycle(group[gi].NativeAssetPrivacy) {
		return eval.executeAssetPrivacyLifecycle(cs, group, uint32(gi), params.TxnGroup[gi].NativeAssetPrivacyProof)
	}
	return eval.executeAssetPrivacyValue(cs, group, uint32(gi), params.TxnGroup[gi].NativeAssetPrivacyProof)
}

func (eval *BlockEvaluator) executeAssetPrivacyValue(cs *roundCowState, group []transactions.Transaction, target uint32, framed []byte) error {
	execution, err := eval.verifyAssetPrivacyExecution(cs, group, target, framed)
	if err != nil {
		return err
	}
	return cs.applyVerifiedAssetPrivacy(execution)
}
