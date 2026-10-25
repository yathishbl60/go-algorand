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
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
)

func (context assetPrivacySpendContext) build(draft spendplan.Statement, records []outputs.Record) (*spendplan.Plan, error) {
	draft.Group = context.group // The caller cannot override the recomputed group digest.
	plan, err := spendplan.Build(draft, context.config)
	if err != nil {
		return nil, err
	}
	if err = plan.CheckOutputs(records); err != nil {
		return nil, err
	}
	for _, record := range records {
		if record.Transaction != context.transaction || record.Round != context.round {
			return nil, spendplan.ErrOutputs
		}
	}
	return plan, nil
}
