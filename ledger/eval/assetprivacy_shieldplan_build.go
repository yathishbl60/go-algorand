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
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/shieldplan"
)

func (context assetPrivacySpendContext) buildShield(body nativebody.Body, records []outputs.Record) (*shieldplan.Plan, error) {
	if body.Operation != nativebody.Shield || len(body.Inputs) != 0 || len(body.Outputs) != 2 || body.Credit != 0 {
		return nil, shieldplan.ErrStatement
	}
	commitments := [2][64]byte{body.Outputs[0].Commitment, body.Outputs[1].Commitment}
	main, err := context.newShieldMain(body, commitments)
	if err != nil {
		return nil, err
	}
	if err = main.CheckOutputs(records); err != nil {
		return nil, err
	}
	for _, record := range records {
		if record.Transaction != context.transaction || record.Round != context.round {
			return nil, shieldplan.ErrOutputs
		}
	}
	return main, nil
}
