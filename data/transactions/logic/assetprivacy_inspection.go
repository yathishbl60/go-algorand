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

package logic

import (
	"fmt"

	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/protocol"
)

func inspectNativePrivacyTransaction(txn *transactions.Transaction) (nativePrivacyInspection, error) {
	if len(txn.NativeAssetPrivacy) == 0 && txn.Type != protocol.NativeAssetPrivacyTx {
		return nativePrivacyInspection{}, nil
	}
	if lifecyclebody.IsLifecycle(txn.NativeAssetPrivacy) {
		return inspectNativePrivacyLifecycle(txn)
	}
	if txn.Type != protocol.NativeAssetPrivacyTx {
		return nativePrivacyInspection{}, fmt.Errorf("native value body requires aprv transaction")
	}
	body, err := nativebody.Inspect(txn.NativeAssetPrivacy)
	return nativePrivacyInspection{family: 1, body: body}, err
}

func inspectNativePrivacyLifecycle(txn *transactions.Transaction) (nativePrivacyInspection, error) {
	body, err := lifecyclebody.Decode(txn.NativeAssetPrivacy)
	if err != nil {
		return nativePrivacyInspection{}, err
	}
	creation := txn.IsNativeAssetPrivacyCreation()
	if (body.Operation == lifecyclebody.Create) != creation || !creation && txn.Type != protocol.NativeAssetPrivacyTx {
		return nativePrivacyInspection{}, fmt.Errorf("invalid native lifecycle transaction placement")
	}
	summary := nativebody.Summary{Operation: nativebody.Kind(body.Operation), Mode: body.Modes,
		Asset: body.Asset, Policy: body.Policy, Suite: body.Suite}
	return nativePrivacyInspection{family: 2, body: summary}, nil
}
