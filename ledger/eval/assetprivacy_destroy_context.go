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
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

func (cs *roundCowState) checkAssetPrivacyDestructionContext(asset basics.AssetIndex, sender basics.Address, record retirement.Record) error {
	if record.Manager != sender {
		return retirement.ErrRecord
	}
	if err := cs.assetPrivacyRetirementAuthority(asset, sender); err != nil {
		return err
	}
	if err := cs.checkAssetPrivacyRetirementSupply(asset); err != nil {
		return err
	}
	return cs.checkAssetPrivacyRetiredPolicy(asset, record)
}

func (cs *roundCowState) checkAssetPrivacyRetiredPolicy(asset basics.AssetIndex, record retirement.Record) error {
	declaration, exists, err := cs.GetAssetPrivacyPolicy(asset)
	if err != nil {
		return err
	}
	if !exists || declaration.Suite != record.Suite {
		return retirement.ErrRecord
	}
	identity, err := declaration.Identity(cs.GenesisHash(), uint64(asset))
	if err != nil {
		return err
	}
	if identity != record.Policy {
		return retirement.ErrRecord
	}
	return nil
}
