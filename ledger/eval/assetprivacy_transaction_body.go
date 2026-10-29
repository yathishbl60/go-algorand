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
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
	"github.com/algorand/go-algorand/protocol"
)

func (cs *roundCowState) assetPrivacyTransactionBodies(group []transactions.Transaction, target uint32) (AssetPrivacySpendBodies, error) {
	if !cs.proto.EnableNativeAssetPrivacy {
		return AssetPrivacySpendBodies{}, policy.ErrDisabled
	}
	if cs.proto.MaxTxnBytesPerBlock <= 0 || len(group) == 0 || len(group) > cs.proto.MaxTxGroupSize || uint64(target) >= uint64(len(group)) {
		return AssetPrivacySpendBodies{}, statement.ErrLimit
	}
	limit := uint64(cs.proto.MaxTxnBytesPerBlock)
	return AssetPrivacySpendBodies{Group: group, Target: target, Limits: statement.Limits{
		MaxGroupSize: cs.proto.MaxTxGroupSize, MaxBodyEstimate: limit, MaxGroupEstimate: limit}}, nil
}

func assetPrivacyTransactionBody(bodies AssetPrivacySpendBodies) (nativebody.Body, error) {
	target := bodies.Group[bodies.Target] // Bounds were checked by assetPrivacyTransactionBodies.
	if target.Type != protocol.NativeAssetPrivacyTx {
		return nativebody.Body{}, nativebody.ErrEncoding
	}
	body, err := nativebody.Decode(target.NativeAssetPrivacy)
	if err != nil {
		return nativebody.Body{}, err
	}
	if err = body.CheckSender([32]byte(target.Sender)); err != nil {
		return nativebody.Body{}, err
	}
	return body, nil
}
