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

package proofbundle

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/proofplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/shieldplan"
)

// Shield joins the no-private-input main proof to every required output role.
// Current native state must supply both main and output plans; this grants no debit.
func Shield(main *shieldplan.Plan, outputs []*proofplan.Plan) (*Plan, error) {
	if main == nil || len(outputs) != 2 {
		return nil, ErrPlan
	}
	plan := &Plan{slots: []Slot{{kind: ShieldKind, output: MainOutput, values: main.Values()}}}
	if err := plan.addOutputs(main.Domain(), main.Outputs(), outputs, -1, Recipient); err != nil {
		return nil, err
	}
	return plan, nil
}
