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
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
)

func (cs *roundCowState) projectAssetPrivacyClosure(body nativebody.Body, records []outputs.Record, bodies AssetPrivacySpendBodies, record *closure.Record) (*AssetPrivacyTransactionPlan, error) {
	plan, err := cs.projectAssetPrivacyTransaction(body, records, bodies)
	if err != nil {
		return nil, err
	}
	plan.closure = record
	return plan, nil
}

func (cs *roundCowState) projectAssetPrivacyValidatedTransaction(body nativebody.Body, bodies AssetPrivacySpendBodies) (*AssetPrivacyTransactionPlan, error) {
	closing, err := cs.assetPrivacyFinalReconciliation(body, bodies)
	if err != nil {
		return nil, err
	}
	if err = cs.checkAssetPrivacyTransactionPolicyMode(body, closing != nil); err != nil {
		return nil, err
	}
	records, err := cs.assetPrivacyTransactionOutputs(body, bodies)
	if err != nil {
		return nil, err
	}
	return cs.projectAssetPrivacyClosure(body, records, bodies, closing)
}
