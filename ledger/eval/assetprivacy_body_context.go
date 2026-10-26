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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

type assetPrivacyBodyContext struct {
	group       [64]byte
	suite       [64]byte
	transaction [32]byte
	sender      basics.Address
	round       uint64
}

func (cs *roundCowState) assetPrivacyBindBodies(suite [64]byte, bodies AssetPrivacySpendBodies) (assetPrivacyBodyContext, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return assetPrivacyBodyContext{}, policy.ErrDisabled
	}
	if bodies.Limits.MaxGroupSize != cs.proto.MaxTxGroupSize {
		return assetPrivacyBodyContext{}, statement.ErrLimit
	}
	group, err := statement.Bind(cs.GenesisHash(), suite, bodies.Target, bodies.Group, bodies.Limits)
	if err != nil {
		return assetPrivacyBodyContext{}, err
	}
	target := bodies.Group[bodies.Target]
	return assetPrivacyBodyContext{group: group, suite: suite, transaction: [32]byte(target.ID()),
		sender: target.Sender, round: uint64(cs.Round())}, nil
}

func (context assetPrivacyBodyContext) proofContext(domain tree.Domain) AssetPrivacyPlanContext {
	return AssetPrivacyPlanContext{Domain: domain, Suite: context.suite, Group: context.group}
}
