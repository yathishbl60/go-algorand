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
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
)

func (cs *roundCowState) prepareAssetPrivacyRegistration(body lifecyclebody.Body) (keys.Record, error) {
	if body.Operation != lifecyclebody.Register || body.ExpectedEpoch == ^uint64(0) {
		return keys.Record{}, keys.ErrEpoch
	}
	domain := keys.Domain{Network: cs.GenesisHash(), Asset: body.Asset, Policy: body.Policy,
		Account: body.Account, Role: body.Role}
	record := keys.Record{Domain: domain, Epoch: body.ExpectedEpoch + 1,
		Created: uint64(cs.Round()), PublicKey: body.PublicKey}
	if err := record.Validate(); err != nil {
		return keys.Record{}, err
	}
	if err := cs.checkAssetPrivacyKeyRole(record); err != nil {
		return keys.Record{}, err
	}
	return record, cs.checkAssetPrivacyRegistrationEpoch(body)
}

func (cs *roundCowState) checkAssetPrivacyRegistrationEpoch(body lifecyclebody.Body) error {
	store, err := cs.openAssetPrivacyKeys(basics.AssetIndex(body.Asset), basics.Address(body.Account), body.Role)
	if err != nil {
		return err
	}
	before, exists, err := store.Current(uint64(cs.Round()))
	if err != nil {
		return err
	}
	if (!exists && body.ExpectedEpoch != 0) || (exists && before.Epoch != body.ExpectedEpoch) {
		return keys.ErrEpoch
	}
	return nil
}
