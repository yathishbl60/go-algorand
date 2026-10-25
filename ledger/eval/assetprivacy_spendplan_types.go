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
	"github.com/algorand/go-algorand/internal/assetprivacy/proofplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/statement"
)

// AssetPrivacySpendBodies carries finalized bounded-decoded ordinary group bodies.
// Limits must come from the pinned profile; estimates are not wire-size ceilings.
type AssetPrivacySpendBodies struct {
	Group  []transactions.Transaction
	Target uint32
	Limits statement.Limits
}

// AssetPrivacySpendPlan is a read-only statement snapshot, never proof acceptance.
type AssetPrivacySpendPlan struct {
	main    *spendplan.Plan
	outputs []*proofplan.Plan
}

func (p *AssetPrivacySpendPlan) Main() *spendplan.Plan { return p.main }
func (p *AssetPrivacySpendPlan) Outputs() []*proofplan.Plan {
	return append([]*proofplan.Plan(nil), p.outputs...)
}

type assetPrivacySpendContext struct {
	config spendplan.Config
	assetPrivacyBodyContext
}
