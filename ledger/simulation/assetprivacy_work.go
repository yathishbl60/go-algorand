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

package simulation

import "github.com/algorand/go-algorand/ledger/ledgercore"

// AfterNativePrivacyWork retains diagnostics even if transaction/group state rolls back.
// Native execution is forbidden in inner transactions, so each event has one root.
func (tracer *evalTracer) AfterNativePrivacyWork(delta ledgercore.NativePrivacyWork) {
	if delta.IsZero() {
		return
	}
	group, txn := tracer.nativePrivacyWorkTarget()
	addNativePrivacyWork(&group.NativePrivacyWork, delta)
	addNativePrivacyWork(&txn.NativePrivacyWork, delta)
}

func (tracer *evalTracer) nativePrivacyWorkTarget() (*TxnGroupResult, *TxnResult) {
	if tracer == nil || tracer.result == nil || len(tracer.result.TxnGroups) != 1 {
		panic("native privacy work requires exactly one simulation group")
	}
	path := tracer.absolutePath()
	group := &tracer.result.TxnGroups[0]
	if len(path) != 1 || path[0] < 0 || path[0] >= len(group.Txns) {
		panic("native privacy work requires an active top-level transaction")
	}
	return group, &group.Txns[path[0]]
}

func addNativePrivacyWork(target **ledgercore.NativePrivacyWork, delta ledgercore.NativePrivacyWork) {
	if *target == nil {
		*target = new(ledgercore.NativePrivacyWork)
	}
	(*target).Add(delta)
}
