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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/shieldplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
)

func assetPrivacySpendKind(body nativebody.Body) spendplan.Kind {
	if body.Mode == policy.Modes(policy.Confidential) {
		if body.Operation == nativebody.Transfer {
			return spendplan.ConfidentialTransfer
		}
		return spendplan.ConfidentialUnshield
	}
	return spendplan.Kind(body.Operation)
}

func (context assetPrivacySpendContext) newShieldMain(body nativebody.Body, commitments [2][64]byte) (*shieldplan.Plan, error) {
	if body.Mode == policy.Modes(policy.Confidential) {
		return shieldplan.NewConfidential(context.config.Domain, context.group, body.Debit, commitments, body.Participants)
	}
	return shieldplan.New(context.config.Domain, context.group, body.Debit, commitments)
}
