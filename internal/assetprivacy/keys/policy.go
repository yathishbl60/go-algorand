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

package keys

import "github.com/algorand/go-algorand/internal/assetprivacy/policy"

// CheckPolicy checks role eligibility, never key ownership or issuer identity.
func (r Role) CheckPolicy(record policy.Record) error {
	if err := record.Validate(); err != nil {
		return err
	}
	if r.allowed(record) {
		return nil
	}
	return ErrRole
}

func (r Role) allowed(record policy.Record) bool {
	switch r {
	case RecipientView, RecipientSpend, ManagerAuthority:
		return true
	case AuditorView:
		return record.Policy.Disclosures&policy.AuditorDisclosure != 0
	case ControllerView:
		return record.Policy.Disclosures&policy.ControllerDisclosure != 0
	case FreezeAuthority:
		return record.Policy.Capabilities&policy.CanFreeze != 0
	case ClawbackAuthority:
		return record.Policy.Capabilities&policy.CanClawback != 0
	}
	return false
}
