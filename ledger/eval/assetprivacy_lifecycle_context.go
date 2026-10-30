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
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
)

func (cs *roundCowState) checkAssetPrivacyLifecycleContext(body lifecyclebody.Body, group []transactions.Transaction, target uint32) ([64]byte, error) {
	if err := body.CheckSender(group[target].Sender); err != nil {
		return [64]byte{}, err
	}
	if _, err := suitecatalog.Lookup(cs.proto, body.Suite); err != nil {
		return [64]byte{}, err
	}
	if body.Operation != lifecyclebody.Adopt {
		if err := cs.checkAssetPrivacyLifecyclePolicy(body); err != nil {
			return [64]byte{}, err
		}
	}
	return cs.assetPrivacyLifecycleBinding(body, group, target)
}

func (cs *roundCowState) assetPrivacyLifecycleBinding(body lifecyclebody.Body, group []transactions.Transaction, target uint32) ([64]byte, error) {
	bodies, err := cs.assetPrivacyTransactionBodies(group, target)
	if err != nil {
		return [64]byte{}, err
	}
	return statement.Bind(cs.GenesisHash(), body.Suite, target, group, bodies.Limits)
}

func (cs *roundCowState) checkAssetPrivacyLifecyclePolicy(body lifecyclebody.Body) error {
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
	return nil
}
