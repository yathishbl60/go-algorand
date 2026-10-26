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

import "github.com/algorand/go-algorand/internal/assetprivacy/tree"

// AssetPrivacyPlanContext snapshots native policy identity and finalized body binding.
// Every field is copied by value; this is expected bridge context, not proof acceptance.
type AssetPrivacyPlanContext struct {
	Domain tree.Domain
	Suite  [64]byte
	Group  [64]byte
}

// Context returns values derived from current policy and the native body Bind.
func (p *AssetPrivacySpendPlan) Context() AssetPrivacyPlanContext {
	if p == nil {
		return AssetPrivacyPlanContext{}
	}
	return p.context
}

func (p *AssetPrivacyRecoveryPlan) Context() AssetPrivacyPlanContext {
	if p == nil {
		return AssetPrivacyPlanContext{}
	}
	return p.context
}

// Plan types keep nested data private and return copies from their slice accessors.
func copyAssetPrivacyPlan[T any](plan *T) *T {
	if plan == nil {
		return nil
	}
	snapshot := *plan
	return &snapshot
}
