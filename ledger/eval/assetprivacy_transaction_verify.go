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
)

// VerifyAssetPrivacyTransaction binds the exact target body to current native state.
// Signatures, public holdings/funding and atomic execution remain separate checks.
func (cs *roundCowState) VerifyAssetPrivacyTransaction(group []transactions.Transaction, target uint32, registry, framed []byte) (*AssetPrivacyTransactionPlan, error) {
	plan, err := cs.AssetPrivacyTransactionProofPlan(group, target)
	if err != nil {
		return nil, err
	}
	context := plan.Context()
	verifier, err := cs.assetPrivacySuiteVerifier(context.Suite, registry)
	if err != nil {
		return nil, err
	}
	if plan.spend != nil {
		err = plan.spend.Verify(verifier, framed)
	} else {
		err = plan.recovery.Verify(verifier, framed)
	}
	if err != nil {
		return nil, err
	}
	return plan, nil
}

func (p *AssetPrivacyTransactionPlan) Context() AssetPrivacyPlanContext {
	if p == nil {
		return AssetPrivacyPlanContext{}
	}
	if p.spend != nil {
		return p.spend.Context()
	}
	if p.recovery != nil {
		return p.recovery.Context()
	}
	return AssetPrivacyPlanContext{}
}
