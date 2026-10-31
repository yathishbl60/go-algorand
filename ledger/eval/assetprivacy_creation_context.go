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
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
)

func (eval *BlockEvaluator) assetPrivacyCreationContext(tx transactions.Transaction, cs *roundCowState, params *logic.EvalParams, gi int) (lifecyclebody.Body, error) {
	group, err := eval.assetPrivacyCreationGroup(tx, params, gi)
	if err != nil {
		return lifecyclebody.Body{}, err
	}
	body, err := cs.assetPrivacyCreationBody(tx)
	if err != nil {
		return lifecyclebody.Body{}, err
	}
	_, err = cs.assetPrivacyLifecycleBinding(body, group, uint32(gi))
	return body, err
}

func (eval *BlockEvaluator) assetPrivacyCreationGroup(tx transactions.Transaction, params *logic.EvalParams, gi int) ([]transactions.Transaction, error) {
	if params == nil || params.GetCaller() != nil || gi < 0 || gi >= len(params.TxnGroup) {
		return nil, errAssetPrivacyAuthorization
	}
	group, err := assetPrivacyExecutionGroup(params.TxnGroup, eval.proto)
	if err != nil {
		return nil, err
	}
	if group[gi].ID() != tx.ID() {
		return nil, errAssetPrivacyAuthorization
	}
	return group, eval.checkAssetPrivacyExecutionFees(params.TxnGroup)
}

func (cs *roundCowState) assetPrivacyCreationBody(tx transactions.Transaction) (lifecyclebody.Body, error) {
	if err := tx.CheckNativeAssetPrivacyCreation(cs.proto); err != nil {
		return lifecyclebody.Body{}, err
	}
	body, err := lifecyclebody.Decode(tx.NativeAssetPrivacy)
	if err != nil {
		return lifecyclebody.Body{}, err
	}
	_, err = suitecatalog.Lookup(cs.proto, body.Suite)
	return body, err
}
