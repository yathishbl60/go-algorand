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

package verify

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/protocol"
)

// Registration additionally needs its possession signature scheme enabled.
// Account authorization may independently use another enabled PQ scheme.
func checkNativePrivacyLifecycleScheme(tx *transactions.SignedTxn, proto config.ConsensusParams) error {
	body, err := lifecyclebody.Decode(tx.Txn.NativeAssetPrivacy)
	if err != nil {
		return err
	}
	if body.Operation == lifecyclebody.Register && body.Role.UsesFalconPossession() && !proto.PQSchemeEnabled(protocol.PQSchemeFalcon1024) {
		return crypto.ErrPQSchemeNotEnabled
	}
	return nil
}
