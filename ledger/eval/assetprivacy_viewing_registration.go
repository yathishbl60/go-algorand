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
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"
	"github.com/algorand/go-algorand/internal/assetprivacy/registrationplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func assetPrivacyRegistrationBundle(record keys.Record, suite, group [64]byte) (*proofbundle.Plan, AssetPrivacyPlanContext, error) {
	if record.Domain.Role == keys.RecipientSpend {
		return assetPrivacyOwnerBundle(record, suite, group)
	}
	return assetPrivacyViewingBundle(record, suite, group)
}

func assetPrivacyViewingBundle(record keys.Record, suite, group [64]byte) (*proofbundle.Plan, AssetPrivacyPlanContext, error) {
	domain := tree.Domain{Genesis: record.Domain.Network, Asset: record.Domain.Asset, Policy: record.Domain.Policy}
	context := AssetPrivacyPlanContext{Domain: domain, Suite: suite, Group: group}
	if !record.Domain.Role.UsesStarkPossession() || record.Domain.Role == keys.RecipientSpend || len(record.PublicKey) != 1184 {
		return nil, context, keys.ErrRole
	}
	var publicKey [1184]byte
	copy(publicKey[:], record.PublicKey)
	main, err := registrationplan.NewViewing(domain, group, publicKey)
	if err != nil {
		return nil, context, err
	}
	bundle, err := proofbundle.ViewingRegistration(main)
	return bundle, context, err
}
