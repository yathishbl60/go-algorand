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

package nativebody

import "github.com/algorand/go-algorand/internal/assetprivacy/policy"

// Summary contains only fixed-size public fields, never envelopes or owned keys.
type Summary struct {
	Operation      Kind
	Mode           policy.Modes
	Asset          uint64
	Policy         [64]byte
	Suite          [64]byte
	Debit          uint64
	Credit         uint64
	DebitSource    [32]byte
	CreditReceiver [32]byte
	InputCount     uint64
	OutputCount    uint64
	Nullifiers     [2][64]byte
	Commitments    [2][64]byte
}

// Inspect validates the complete canonical body without copying ciphertexts.
// It performs no authorization, ledger lookup or proof verification.
func Inspect(encoded []byte) (Summary, error) {
	parsed, err := preflight(encoded)
	if err != nil {
		return Summary{}, err
	}
	b := parsed.header
	result := Summary{Operation: b.Operation, Mode: b.Mode, Asset: b.Asset, Policy: b.Policy, Suite: b.Suite,
		Debit: b.Debit, Credit: b.Credit, DebitSource: b.DebitSource, CreditReceiver: b.CreditReceiver,
		InputCount: uint64(parsed.inputCount), OutputCount: uint64(parsed.outputCount)}
	for i := 0; i < parsed.inputCount; i++ {
		result.Nullifiers[i] = parsed.inputs[i].Nullifier
	}
	for i := 0; i < parsed.outputCount; i++ {
		result.Commitments[i] = parsed.outputs[i].Commitment
	}
	return result, nil
}
