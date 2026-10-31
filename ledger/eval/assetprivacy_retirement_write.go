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
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
)

func (cs *roundCowState) retireAssetPrivacy(body lifecyclebody.Body, sender basics.Address) error {
	asset := basics.AssetIndex(body.Asset)
	if err := cs.assetPrivacyRetirementAuthority(asset, sender); err != nil {
		return err
	}
	if err := cs.checkAssetPrivacyActive(asset); err != nil {
		return err
	}
	if err := cs.checkAssetPrivacyRetirementSupply(asset); err != nil {
		return err
	}
	record := retirement.Record{Manager: sender, Policy: body.Policy, Suite: body.Suite,
		Created: uint64(cs.Round()), Epoch: 1}
	return cs.writeAssetPrivacyRetirement(asset, record)
}

func (cs *roundCowState) writeAssetPrivacyRetirement(asset basics.AssetIndex, record retirement.Record) error {
	key, err := retirement.Key(uint64(asset))
	if err != nil {
		return err
	}
	encoded, err := retirement.Encode(record)
	if err != nil {
		return err
	}
	return cs.kvPut(key, encoded)
}
