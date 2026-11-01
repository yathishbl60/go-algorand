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
	"errors"
	"fmt"

	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
)

const nativePrivacyAVMVersion = 14

type nativePrivacyInspection struct {
	family uint64
	body   nativebody.Summary
}

func (cx *EvalContext) nativePrivacyTxnField(txn *transactions.Transaction, field TxnField, index uint64) (stackValue, error) {
	if field < NativePrivacyFamily || field > NativePrivacyOutputCommitments {
		return stackValue{}, fmt.Errorf("invalid txn field %s", field)
	}
	if cx.EvalParams == nil || cx.Proto == nil || cx.version < nativePrivacyAVMVersion || !cx.Proto.NativeAssetPrivacyExecutionConfigured() {
		return stackValue{}, errors.New("native privacy AVM fields are disabled")
	}
	value, err := inspectNativePrivacyTransaction(txn)
	if err != nil {
		return stackValue{}, err
	}
	return value.field(field, index)
}
