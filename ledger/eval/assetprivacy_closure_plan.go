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
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func (cs *roundCowState) assetPrivacyFinalReconciliation(body nativebody.Body, bodies AssetPrivacySpendBodies) (*closure.Record, error) {
	declaration, exists, err := cs.GetAssetPrivacyPolicy(basics.AssetIndex(body.Asset))
	if err != nil {
		return nil, err
	}
	if !exists {
		return nil, policy.ErrMissing
	}
	if declaration.Policy.Modes.Allows(policy.Public) || body.Credit == 0 {
		return nil, nil
	}
	if body.Operation != nativebody.Unshield && body.Operation != nativebody.FullPublic {
		return nil, closure.ErrRecord
	}
	return cs.buildAssetPrivacyClosure(body, bodies)
}

func (cs *roundCowState) buildAssetPrivacyClosure(body nativebody.Body, bodies AssetPrivacySpendBodies) (*closure.Record, error) {
	asset := basics.AssetIndex(body.Asset)
	params, creator, err := cs.AssetParams(asset)
	if err != nil {
		return nil, err
	}
	return cs.finishAssetPrivacyClosure(body, bodies, params, creator)
}

func (cs *roundCowState) finishAssetPrivacyClosure(body nativebody.Body, bodies AssetPrivacySpendBodies, params basics.AssetParams, creator basics.Address) (*closure.Record, error) {
	asset := basics.AssetIndex(body.Asset)
	if params.Manager.IsZero() || body.CreditReceiver != creator || body.Debit != 0 {
		return nil, closure.ErrRecord
	}
	if err := cs.checkAssetPrivacyFinalSupply(asset, creator, body.Credit); err != nil {
		return nil, err
	}
	destroy, err := assetPrivacyClosingTransaction(body, bodies, params.Manager)
	if err != nil {
		return nil, err
	}
	context, err := cs.assetPrivacyBindBodies(body.Suite, bodies)
	if err != nil {
		return nil, err
	}
	record := &closure.Record{Manager: params.Manager, Creator: creator, Policy: body.Policy, Suite: body.Suite,
		Group: context.group, Destroy: [32]byte(destroy.ID()), Total: params.Total, Created: uint64(cs.Round()), Epoch: 1}
	return record, record.Validate()
}
