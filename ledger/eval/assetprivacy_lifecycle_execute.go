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
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
)

// Current top-level PQ authorization is checked before fee/rekey application.
// The caller owns the disposable execution child; these operations need no STARK.
func (eval *BlockEvaluator) executeAssetPrivacyLifecycle(cs *roundCowState, group []transactions.Transaction, target uint32, framed []byte) error {
	if uint64(target) >= uint64(len(group)) {
		return lifecyclebody.ErrEncoding
	}
	body, err := lifecyclebody.Decode(group[target].NativeAssetPrivacy)
	if err != nil {
		return err
	}
	binding, err := cs.checkAssetPrivacyLifecycleContext(body, group, target)
	if err != nil {
		return err
	}
	return eval.executeAssetPrivacyLifecycleBody(cs, body, group[target].Sender, binding, framed)
}

func (eval *BlockEvaluator) executeAssetPrivacyLifecycleBody(cs *roundCowState, body lifecyclebody.Body, sender basics.Address, binding [64]byte, framed []byte) error {
	if body.Operation == lifecyclebody.Register {
		return eval.registerAssetPrivacyLifecycle(cs, body, binding, framed)
	}
	if len(framed) != 0 {
		return lifecyclebody.ErrEncoding
	}
	return cs.applyAssetPrivacyLifecycle(body, sender)
}

func (cs *roundCowState) applyAssetPrivacyLifecycle(body lifecyclebody.Body, sender basics.Address) error {
	asset := basics.AssetIndex(body.Asset)
	switch body.Operation {
	case lifecyclebody.Adopt:
		return cs.adoptAssetPrivacyLifecycle(body, sender)
	case lifecyclebody.Assign:
		_, err := cs.SetAssetPrivacyDisclosureTargets(asset, sender, basics.Address(body.Auditor), body.ExpectedEpoch)
		return err
	case lifecyclebody.Freeze:
		_, err := cs.SetAssetPrivacyFrozen(asset, basics.Address(body.Target), sender, body.ExpectedEpoch, body.Frozen)
		return err
	default:
		return lifecyclebody.ErrOperation
	}
}
