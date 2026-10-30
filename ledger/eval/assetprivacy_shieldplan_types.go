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
	"github.com/algorand/go-algorand/internal/assetprivacy/proofplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/shieldplan"
)

// AssetPrivacyShieldPlan snapshots current expected statements, not acceptance.
type AssetPrivacyShieldPlan struct {
	main    *shieldplan.Plan
	outputs []*proofplan.Plan
	context AssetPrivacyPlanContext
}

func (p *AssetPrivacyShieldPlan) Main() *shieldplan.Plan {
	if p == nil {
		return nil
	}
	return copyAssetPrivacyPlan(p.main)
}

func (p *AssetPrivacyShieldPlan) Outputs() []*proofplan.Plan {
	if p == nil || p.outputs == nil {
		return nil
	}
	snapshots := make([]*proofplan.Plan, len(p.outputs))
	for index, output := range p.outputs {
		snapshots[index] = copyAssetPrivacyPlan(output)
	}
	return snapshots
}

func (p *AssetPrivacyShieldPlan) Context() AssetPrivacyPlanContext {
	if p == nil {
		return AssetPrivacyPlanContext{}
	}
	return p.context
}
