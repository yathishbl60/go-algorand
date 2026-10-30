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
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
)

// AssetPrivacyShieldProofPlan derives its debit/commitments from the exact target.
// It neither verifies proofs nor authorizes a debit, append or state mutation.
func (cs *roundCowState) AssetPrivacyShieldProofPlan(records []outputs.Record, bodies AssetPrivacySpendBodies) (*AssetPrivacyShieldPlan, error) {
	canonical, err := cs.assetPrivacyTransactionBodies(bodies.Group, bodies.Target)
	if err != nil {
		return nil, err
	}
	body, err := assetPrivacyTransactionBody(canonical)
	if err != nil {
		return nil, err
	}
	if body.Operation != nativebody.Shield {
		return nil, nativebody.ErrArity
	}
	if err = cs.checkAssetPrivacyTransactionPolicy(body); err != nil {
		return nil, err
	}
	return cs.assetPrivacyShieldProofState(body, records, canonical)
}

func (cs *roundCowState) assetPrivacyShieldProofState(body nativebody.Body, records []outputs.Record, bodies AssetPrivacySpendBodies) (*AssetPrivacyShieldPlan, error) {
	asset := basics.AssetIndex(body.Asset)
	context, err := cs.assetPrivacySpendContext(asset, bodies)
	if err != nil {
		return nil, err
	}
	main, err := context.buildShield(body, records)
	if err != nil {
		return nil, err
	}
	return cs.assetPrivacyShieldOutputPlans(asset, main, records, context.proofContext(main.Domain()))
}
