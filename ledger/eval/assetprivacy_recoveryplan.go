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
)

// AssetPrivacyRecoveryProofPlan checks current identities and derives expected vectors.
// Header.Sender equality is not signature authentication. Ordinary/PQ authorization,
// native body/mode checks, proof verification, funding and execution remain required.
func (cs *roundCowState) AssetPrivacyRecoveryProofPlan(asset basics.AssetIndex, draft recoveryplan.Statement, records []outputs.Record, bodies AssetPrivacySpendBodies) (*AssetPrivacyRecoveryPlan, error) {
	context, err := cs.assetPrivacyRecoveryContext(asset, bodies)
	if err != nil {
		return nil, err
	}
	main, err := context.build(draft, records)
	if err != nil {
		return nil, err
	}
	if err = cs.checkAssetPrivacyRecoveryMode(asset, main.Kind()); err != nil {
		return nil, err
	}
	if err = cs.checkAssetPrivacySpendInput(asset, context.config.Domain.Policy, main.Input()); err != nil {
		return nil, err
	}
	return cs.assetPrivacyRecoveryOutputPlans(asset, main, records)
}

func (cs *roundCowState) assetPrivacyRecoveryOutputPlans(asset basics.AssetIndex, main *recoveryplan.Plan, records []outputs.Record) (*AssetPrivacyRecoveryPlan, error) {
	plan := &AssetPrivacyRecoveryPlan{main: main}
	for index, record := range records {
		output, err := cs.assetPrivacyRecoveryOutputPlan(asset, main, index, record)
		if err != nil {
			return nil, err
		}
		plan.outputs = append(plan.outputs, output)
	}
	return plan, nil
}
