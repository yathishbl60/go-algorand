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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/recoveryplan"
)

// Private-only credit requires the exact final reconciliation/destruction pair.
func (cs *roundCowState) checkAssetPrivacyRecoveryMode(asset basics.AssetIndex, kind recoveryplan.Kind, bodies AssetPrivacySpendBodies) error {
	declaration, err := cs.assetPrivacyRecoveryDeclaration(asset)
	if err != nil {
		return err
	}
	modes := declaration.Policy.Modes
	mode := policy.Shielded
	if kind.Confidential() {
		mode = policy.Confidential
	}
	if !modes.Allows(mode) {
		return policy.ErrModes
	}
	public := kind.Public()
	if public && !modes.Allows(policy.Public) {
		return cs.checkAssetPrivacyClosingRecovery(asset, kind, bodies)
	}
	return nil
}
