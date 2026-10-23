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
	"github.com/algorand/go-algorand/internal/assetprivacy/control"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func (cs *roundCowState) checkAssetPrivacyFreezeAuthority(asset basics.AssetIndex, authority basics.Address) error {
	declaration, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return err
	}
	if !exists {
		return policy.ErrMissing
	}
	if declaration.Policy.Capabilities&policy.CanFreeze == 0 {
		return control.ErrCapability
	}
	params, _, err := cs.AssetParams(asset)
	if err != nil {
		return err
	}
	if params.Freeze.IsZero() || params.Freeze != authority {
		return control.ErrAuthority
	}
	return nil
}
