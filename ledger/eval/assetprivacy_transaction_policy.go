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
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func (cs *roundCowState) checkAssetPrivacyTransactionPolicy(body nativebody.Body) error {
	record, exists, err := cs.GetAssetPrivacyPolicy(basics.AssetIndex(body.Asset))
	if err != nil {
		return err
	}
	if !exists {
		return policy.ErrMissing
	}
	identity, err := record.Identity(cs.GenesisHash(), body.Asset)
	if err != nil {
		return err
	}
	if identity != body.Policy || record.Suite != body.Suite {
		return policy.ErrRecord
	}
	if err = checkAssetPrivacyTransactionModes(record.Policy, body); err != nil {
		return err
	}
	return cs.checkAssetPrivacyIssuerInventory(record.Policy, body)
}

func checkAssetPrivacyTransactionModes(current policy.Policy, body nativebody.Body) error {
	if current.Modes&body.Mode != body.Mode {
		return policy.ErrModes
	}
	if body.Credit != 0 && !current.Modes.Allows(policy.Public) {
		return policy.ErrModes
	}
	if body.Debit != 0 && body.Operation != nativebody.Shield && !current.Modes.Allows(policy.Public) {
		return policy.ErrModes
	}
	for _, output := range body.Outputs {
		if output.Disclosures != current.Disclosures {
			return policy.ErrRecord
		}
	}
	return nil
}
