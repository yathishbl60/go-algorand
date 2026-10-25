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

package proofplan

import "github.com/algorand/go-algorand/internal/assetprivacy/outputs"

// BuildRemainder derives the weaker viewing-only recipient statement for a
// preauthorized original-owner recovery remainder. It grants no recovery power.
// The main recovery proof must preserve the original account/owner, and the
// native verifier must enforce current clawback authority and required signatures.
// Ordinary outputs must use Build; a proof may never select this weaker kind.
func BuildRemainder(record outputs.Record, config Config) (*Plan, error) {
	plan, err := Build(record, config)
	if err != nil {
		return nil, err
	}
	plan.slots[0].kind = RetainedRecipientProof
	plan.slots[0].values = plan.slots[0].values[:RetainedRecipientPublicValues:RetainedRecipientPublicValues]
	return plan, nil
}
