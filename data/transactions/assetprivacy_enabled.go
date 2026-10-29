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

package transactions

import (
	"errors"

	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/protocol"
)

func (tx Transaction) checkNativeAssetPrivacy(proto config.ConsensusParams) error {
	if tx.Type != protocol.NativeAssetPrivacyTx {
		if len(tx.NativeAssetPrivacy) != 0 {
			return errors.New("orphan native asset privacy body")
		}
		return nil
	}
	if !proto.NativeAssetPrivacyExecutionConfigured() {
		return policy.ErrDisabled
	}
	body, err := nativebody.Decode(tx.NativeAssetPrivacy)
	if err != nil {
		return err
	}
	return body.CheckSender([32]byte(tx.Sender))
}
