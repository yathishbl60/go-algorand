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
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/recoveryplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
)

func assetPrivacyTransactionInput(input nativebody.Input) spendplan.Input {
	return spendplan.Input{Nullifier: input.Nullifier, Root: input.Root, Count: input.Count}
}

func assetPrivacyTransactionCommitments(body nativebody.Body) [][64]byte {
	values := make([][64]byte, len(body.Outputs))
	for i, output := range body.Outputs {
		values[i] = output.Commitment
	}
	return values
}

func (cs *roundCowState) projectAssetPrivacyTransaction(body nativebody.Body, records []outputs.Record, bodies AssetPrivacySpendBodies) (*AssetPrivacyTransactionPlan, error) {
	plan := &AssetPrivacyTransactionPlan{conversion: AssetPrivacyPublicConversion{
		Asset: basics.AssetIndex(body.Asset), Debit: body.Debit, DebitSource: body.DebitSource,
		Credit: body.Credit, CreditReceiver: body.CreditReceiver}}
	var err error
	if body.Operation == nativebody.Shield {
		plan.shield, err = cs.AssetPrivacyShieldProofPlan(records, bodies)
	} else if body.Operation == nativebody.Transfer || body.Operation == nativebody.Unshield {
		plan.spend, err = cs.projectAssetPrivacySpend(body, records, bodies)
	} else {
		plan.recovery, err = cs.projectAssetPrivacyRecovery(body, records, bodies)
	}
	return plan, err
}

func (cs *roundCowState) projectAssetPrivacySpend(body nativebody.Body, records []outputs.Record, bodies AssetPrivacySpendBodies) (*AssetPrivacySpendPlan, error) {
	draft := spendplan.Statement{Kind: spendplan.Kind(body.Operation), Debit: body.Debit,
		Credit: body.Credit, Outputs: assetPrivacyTransactionCommitments(body), Inputs: [2]spendplan.Input{
			assetPrivacyTransactionInput(body.Inputs[0]), assetPrivacyTransactionInput(body.Inputs[1])}}
	return cs.AssetPrivacySpendProofPlan(basics.AssetIndex(body.Asset), draft, records, bodies)
}

func (cs *roundCowState) projectAssetPrivacyRecovery(body nativebody.Body, records []outputs.Record, bodies AssetPrivacySpendBodies) (*AssetPrivacyRecoveryPlan, error) {
	draft := recoveryplan.Statement{Kind: recoveryplan.Kind(body.Operation - 3), Credit: body.Credit,
		Outputs: assetPrivacyTransactionCommitments(body), Input: assetPrivacyTransactionInput(body.Inputs[0])}
	return cs.AssetPrivacyRecoveryProofPlan(basics.AssetIndex(body.Asset), draft, records, bodies)
}
