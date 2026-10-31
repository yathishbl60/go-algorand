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
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func (cs *roundCowState) initializeAssetPrivacyCreation(tx transactions.Transaction, body lifecyclebody.Body, asset basics.AssetIndex) error {
	params, creator, err := cs.AssetParams(asset)
	if err != nil {
		return err
	}
	if creator != tx.Sender || params != tx.AssetParams || asset == 0 {
		return policy.ErrRecord
	}
	record, err := cs.assetPrivacyCreationRecord(asset, body, params)
	if err != nil {
		return err
	}
	if err = cs.writeAssetPrivacyPolicy(asset, record); err != nil {
		return err
	}
	return cs.initializeAssetPrivacyCreationTargets(asset, body, params)
}

func (cs *roundCowState) assetPrivacyCreationRecord(asset basics.AssetIndex, body lifecyclebody.Body, params basics.AssetParams) (policy.Record, error) {
	if _, exists, err := cs.GetAssetPrivacyPolicy(asset); err != nil || exists {
		if err != nil {
			return policy.Record{}, err
		}
		return policy.Record{}, policy.ErrAlreadyAdopted
	}
	if err := cs.checkAssetPrivacyAdoptionSupply(asset); err != nil {
		return policy.Record{}, err
	}
	declaration := policy.Policy{Modes: body.Modes, Capabilities: body.Capabilities, Disclosures: body.Disclosures}
	record := policy.Record{Policy: declaration, InitialCapabilities: body.Capabilities, Suite: body.Suite, Created: uint64(cs.Round())}
	return record, policy.CheckCreation(cs.proto.EnableNativeAssetPrivacy, declaration, privacyAssetCapabilities(params))
}

func (cs *roundCowState) initializeAssetPrivacyCreationTargets(asset basics.AssetIndex, body lifecyclebody.Body, params basics.AssetParams) error {
	store, err := cs.openAssetPrivacyDisclosureTargets(asset)
	if err != nil {
		return err
	}
	_, err = store.Set(body.Auditor, params.Clawback, 0, uint64(cs.Round()))
	return err
}
