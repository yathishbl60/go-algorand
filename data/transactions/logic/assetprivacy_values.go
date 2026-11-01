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

import "fmt"

func (v nativePrivacyInspection) field(field TxnField, index uint64) (stackValue, error) {
	switch field {
	case NativePrivacyDebitSource:
		return stackValue{Bytes: v.body.DebitSource[:]}, nil
	case NativePrivacyCreditReceiver:
		return stackValue{Bytes: v.body.CreditReceiver[:]}, nil
	case NativePrivacyPolicy:
		return stackValue{Bytes: v.body.Policy[:]}, nil
	case NativePrivacySuite:
		return stackValue{Bytes: v.body.Suite[:]}, nil
	case NativePrivacyInputNullifiers, NativePrivacyOutputCommitments:
		return v.array(field, index)
	default:
		return v.number(field)
	}
}

func (v nativePrivacyInspection) number(field TxnField) (stackValue, error) {
	values := map[TxnField]uint64{
		NativePrivacyFamily:     v.family,
		NativePrivacyAsset:      v.body.Asset,
		NativePrivacyMode:       uint64(v.body.Mode),
		NativePrivacyOperation:  uint64(v.body.Operation),
		NativePrivacyDebit:      v.body.Debit,
		NativePrivacyCredit:     v.body.Credit,
		NativePrivacyNumInputs:  v.body.InputCount,
		NativePrivacyNumOutputs: v.body.OutputCount,
	}
	value, ok := values[field]
	if !ok {
		return stackValue{}, fmt.Errorf("invalid native privacy txn field %s", field)
	}
	return stackValue{Uint: value}, nil
}

func (v nativePrivacyInspection) array(field TxnField, index uint64) (stackValue, error) {
	if field == NativePrivacyInputNullifiers && index < v.body.InputCount {
		return stackValue{Bytes: v.body.Nullifiers[index][:]}, nil
	}
	if field == NativePrivacyOutputCommitments && index < v.body.OutputCount {
		return stackValue{Bytes: v.body.Commitments[index][:]}, nil
	}
	return stackValue{}, fmt.Errorf("invalid %s index %d", field, index)
}
