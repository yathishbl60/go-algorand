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
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"
	"github.com/algorand/go-algorand/internal/assetprivacy/registrationplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func (eval *BlockEvaluator) registerAssetPrivacyLifecycle(cs *roundCowState, body lifecyclebody.Body, binding [64]byte, framed []byte) error {
	if body.Role != keys.RecipientSpend {
		return cs.registerAssetPrivacyAdmin(body, binding, framed)
	}
	record, err := cs.prepareAssetPrivacyRegistration(body)
	if err != nil {
		return err
	}
	if err = cs.reserveAssetPrivacyProofChecks(1); err != nil {
		return err
	}
	if err = eval.verifyAssetPrivacyOwner(cs, record, body.Suite, binding, framed); err != nil {
		return err
	}
	return cs.PutAssetPrivacyKey(record)
}

func (eval *BlockEvaluator) verifyAssetPrivacyOwner(cs *roundCowState, record keys.Record, suite, group [64]byte, framed []byte) error {
	bundle, context, err := assetPrivacyOwnerBundle(record, suite, group)
	if err != nil {
		return err
	}
	registry, err := eval.assetPrivacyRegistry(suite)
	if err != nil {
		return err
	}
	verifier, err := cs.assetPrivacySuiteVerifier(suite, registry)
	if err != nil {
		return err
	}
	return verifier.Verify(bundle, privacyBridgeContext(context), framed)
}

func assetPrivacyOwnerBundle(record keys.Record, suite, group [64]byte) (*proofbundle.Plan, AssetPrivacyPlanContext, error) {
	domain := tree.Domain{Genesis: record.Domain.Network, Asset: record.Domain.Asset, Policy: record.Domain.Policy}
	context := AssetPrivacyPlanContext{Domain: domain, Suite: suite, Group: group}
	if record.Domain.Role != keys.RecipientSpend || len(record.PublicKey) != 64 {
		return nil, context, keys.ErrRole
	}
	var owner [64]byte
	copy(owner[:], record.PublicKey)
	main, err := registrationplan.New(domain, group, owner)
	if err != nil {
		return nil, context, err
	}
	bundle, err := proofbundle.RecipientSpendRegistration(main)
	return bundle, context, err
}
