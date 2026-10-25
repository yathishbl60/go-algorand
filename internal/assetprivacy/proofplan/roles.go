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

import "github.com/algorand/go-algorand/internal/assetprivacy/envelope"

// CheckRoles is a structural exact-order check only; it never verifies a proof.
// Recipient precedes optional auditor, then optional controller. Empty plans fail.
func (p *Plan) CheckRoles(roles []envelope.Role) error {
	if p == nil || len(p.slots) == 0 || len(roles) != len(p.slots) {
		return ErrRoles
	}
	for index, slot := range p.slots {
		if roles[index] != slot.role {
			return ErrRoles
		}
	}
	return nil
}
