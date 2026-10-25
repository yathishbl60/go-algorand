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

// BuildRecoveryRecipient selects current recipient keys without ordinary freeze checks.
// It is restricted to preauthorized issuer-recovery receipts, never ordinary outputs.
// Full native current-clawback signatures and the recovery main proof remain required.
func BuildRecoveryRecipient(record outputs.Record, config Config) (*Plan, error) {
	plan, err := Build(record, config)
	if err != nil {
		return nil, err
	}
	plan.slots[0].kind = RecoveryRecipientProof
	plan.slots[0].values = plan.slots[0].values[:RecoveryRecipientPublicValues:RecoveryRecipientPublicValues]
	return plan, nil
}
