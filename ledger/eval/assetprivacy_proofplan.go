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
	"github.com/algorand/go-algorand/internal/assetprivacy/proofplan"
)

// AssetPrivacyOutputProofPlan derives expected statements from current native state.
// It neither verifies proofs nor authorizes output append or transaction acceptance.
// Rebuild after prior group mutations; caller still binds finalized bodies/ordering.
func (cs *roundCowState) AssetPrivacyOutputProofPlan(asset basics.AssetIndex, record outputs.Record) (*proofplan.Plan, error) {
	config, err := cs.assetPrivacyOutputProofConfig(asset)
	if err != nil {
		return nil, err
	}
	if err = cs.checkAssetPrivacyNewOutput(asset, record); err != nil {
		return nil, err
	}
	return proofplan.Build(record, config)
}
