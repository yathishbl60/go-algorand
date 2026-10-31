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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
	"github.com/algorand/go-algorand/protocol"
)

func (cs *roundCowState) checkAssetPrivacyRetiredWrite(asset uint64, key string, present bool) error {
	if !present {
		return nil
	}
	_, retired, err := cs.readAssetPrivacyRetirement(basics.AssetIndex(asset))
	if err != nil {
		return err
	}
	marker, _ := retirement.Key(asset)
	if retired && key != marker {
		return retirement.ErrRetired
	}
	return nil
}

func (cs *roundCowState) assetPrivacyExistingNeedsPQ(tx transactions.Transaction, asset basics.AssetIndex, admin bool) (bool, error) {
	_, retired, err := cs.readAssetPrivacyRetirement(asset)
	if err != nil {
		return false, err
	}
	destroy := tx.Type == protocol.AssetConfigTx && tx.AssetParams == (basics.AssetParams{})
	if retired && !destroy {
		return false, retirement.ErrRetired
	}
	record, exists, err := cs.privacyHoldingPolicy(asset)
	if err != nil || !exists {
		return false, err
	}
	if tx.Type == protocol.AssetTransferTx && !record.Policy.Modes.Allows(policy.Public) {
		return false, policy.ErrModes
	}
	return admin, nil
}
