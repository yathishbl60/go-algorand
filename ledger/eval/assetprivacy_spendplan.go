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
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
)

// AssetPrivacySpendProofPlan recomputes body context and current expected statements.
// It performs no proof verification or state writes. Native body parsing, modes,
// consent, funding, work limits and complete transaction authorization remain required.
func (cs *roundCowState) AssetPrivacySpendProofPlan(asset basics.AssetIndex, draft spendplan.Statement, records []outputs.Record, bodies AssetPrivacySpendBodies) (*AssetPrivacySpendPlan, error) {
	context, err := cs.assetPrivacySpendContext(asset, bodies)
	if err != nil {
		return nil, err
	}
	main, err := context.build(draft, records)
	if err != nil {
		return nil, err
	}
	if err = cs.checkAssetPrivacySpendInputs(asset, context.config.Domain.Policy, main.Inputs()); err != nil {
		return nil, err
	}
	native := context.proofContext(context.config.Domain)
	return cs.assetPrivacySpendOutputPlans(asset, main, records, native)
}

func (cs *roundCowState) assetPrivacySpendOutputPlans(asset basics.AssetIndex, main *spendplan.Plan, records []outputs.Record, context AssetPrivacyPlanContext) (*AssetPrivacySpendPlan, error) {
	if main == nil || main.Domain() != context.Domain {
		return nil, spendplan.ErrConfig
	}
	plan := &AssetPrivacySpendPlan{main: main, context: context}
	for _, record := range records {
		output, err := cs.AssetPrivacyOutputProofPlan(asset, record)
		if err != nil {
			return nil, err
		}
		plan.outputs = append(plan.outputs, output)
	}
	return plan, nil
}
