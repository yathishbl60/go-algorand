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
)

// AssetPrivacyControlView exposes reads without allowing marker/root divergence.
type AssetPrivacyControlView struct{ store *control.Store }

// OpenAssetPrivacyControl opens a read-only native view, not an authorization API.
func (cs *roundCowState) OpenAssetPrivacyControl(asset basics.AssetIndex, account basics.Address) (*AssetPrivacyControlView, error) {
	store, err := cs.openAssetPrivacyControl(asset, account)
	if err != nil {
		return nil, err
	}
	return &AssetPrivacyControlView{store}, nil
}

func (v *AssetPrivacyControlView) Current(round uint64) (control.Record, bool, error) {
	return v.store.Current(round)
}

func (v *AssetPrivacyControlView) Epoch(epoch, round uint64) (control.Record, bool, error) {
	return v.store.Epoch(epoch, round)
}
