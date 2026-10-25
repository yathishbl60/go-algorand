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
	"github.com/algorand/go-algorand/internal/assetprivacy/recoveryplan"
)

// Recovery fixes the weaker remainder kind only at the main plan's retained index.
// Current ordinary/PQ issuer signatures remain a separate native requirement.
func Recovery(main *recoveryplan.Plan, outputs []*proofplan.Plan) (*Plan, error) {
	if main == nil || main.Kind() < recoveryplan.PartialPrivate || main.Kind() > recoveryplan.FullPublic {
		return nil, ErrPlan
	}
	if len(outputs) != len(main.Outputs()) {
		return nil, ErrPlan
	}
	kind := Kind(main.Kind()) + 3
	plan := &Plan{slots: []Slot{{kind: kind, output: MainOutput, values: main.Values()}}}
	remainder := -1
	if index, ok := main.RemainderIndex(); ok {
		remainder = index
	}
	if err := plan.addOutputs(main.Domain(), main.Outputs(), outputs, remainder, RecoveryRecipient); err != nil {
		return nil, err
	}
	return plan, nil
}
