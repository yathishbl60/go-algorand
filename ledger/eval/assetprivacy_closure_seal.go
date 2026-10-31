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
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

// Called only after all native proofs and value changes succeed in this child.
func (cs *roundCowState) sealAssetPrivacyClosure(plan *AssetPrivacyTransactionPlan) error {
	if plan.closure == nil {
		return nil
	}
	asset, record := plan.conversion.Asset, *plan.closure
	if record.Group != plan.Context().Group {
		return retirement.ErrRecord
	}
	if err := cs.checkAssetPrivacyRetirementSupply(asset); err != nil {
		return err
	}
	if err := cs.writeAssetPrivacyClosure(asset, record); err != nil {
		return err
	}
	marker := retirement.Record{Manager: record.Manager, Policy: record.Policy, Suite: record.Suite,
		Created: record.Created, Epoch: 1}
	return cs.writeAssetPrivacyRetirement(asset, marker)
}
