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
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/recoveryplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
)

// VerifyAssetPrivacySpend selects consensus pins/budgets and checks an exact native plan.
// No caller-selected verifier is accepted. Consent, body matching and application remain.
func (cs *roundCowState) VerifyAssetPrivacySpend(asset basics.AssetIndex, draft spendplan.Statement, records []outputs.Record, bodies AssetPrivacySpendBodies, registry, framed []byte) (*AssetPrivacySpendPlan, error) {
	plan, err := cs.AssetPrivacySpendProofPlan(asset, draft, records, bodies)
	if err != nil {
		return nil, err
	}
	verifier, err := cs.assetPrivacySuiteVerifier(plan.Context().Suite, registry)
	if err != nil {
		return nil, err
	}
	if err = plan.Verify(verifier, framed); err != nil {
		return nil, err
	}
	return plan, nil
}

func (cs *roundCowState) VerifyAssetPrivacyRecovery(asset basics.AssetIndex, draft recoveryplan.Statement, records []outputs.Record, bodies AssetPrivacySpendBodies, registry, framed []byte) (*AssetPrivacyRecoveryPlan, error) {
	plan, err := cs.AssetPrivacyRecoveryProofPlan(asset, draft, records, bodies)
	if err != nil {
		return nil, err
	}
	verifier, err := cs.assetPrivacySuiteVerifier(plan.Context().Suite, registry)
	if err != nil {
		return nil, err
	}
	if err = plan.Verify(verifier, framed); err != nil {
		return nil, err
	}
	return plan, nil
}
