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

func (cs *roundCowState) assetPrivacyRecoveryDeclaration(asset basics.AssetIndex) (policy.Record, error) {
	declaration, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return policy.Record{}, err
	}
	if !exists {
		return policy.Record{}, policy.ErrMissing
	}
	if declaration.Policy.Capabilities&policy.CanClawback == 0 || declaration.Policy.Disclosures&policy.ControllerDisclosure == 0 {
		return policy.Record{}, recoveryplan.ErrAuthority
	}
	return declaration, nil
}

func (cs *roundCowState) assetPrivacyCurrentClawback(asset basics.AssetIndex) ([32]byte, error) {
	params, _, err := cs.AssetParams(asset)
	if err != nil {
		return [32]byte{}, err
	}
	if params.Clawback.IsZero() {
		return [32]byte{}, recoveryplan.ErrAuthority
	}
	return params.Clawback, nil
}
