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

package ledger

import (
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/disclosure"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
)

func (context nativePrivacyAuthorizationQuery) targets() (*disclosure.Record, error) {
	domain := disclosure.Domain{Network: context.domain.Genesis, Asset: context.domain.Asset, Policy: context.domain.Policy}
	flags := disclosure.Flags(context.snapshot.Policy.Policy.Disclosures)
	store, err := disclosure.Open(query.ReadOnly[disclosure.Write]{Reader: context.kv}, domain, flags)
	if err != nil {
		return nil, err
	}
	record, exists, err := store.Current(context.snapshot.Round)
	if err != nil {
		return nil, err
	}
	return context.checkTargets(record, exists)
}

func (context nativePrivacyAuthorizationQuery) checkTargets(record disclosure.Record, exists bool) (*disclosure.Record, error) {
	flags := disclosure.Flags(context.snapshot.Policy.Policy.Disclosures)
	if !exists {
		if flags != 0 {
			return nil, disclosure.ErrMissing
		}
		return nil, nil
	}
	if record.Flags&disclosure.ControllerRequired != 0 && !context.params.Clawback.IsZero() && record.Controller != context.params.Clawback {
		return nil, disclosure.ErrTarget
	}
	return &record, nil
}

func (context nativePrivacyAuthorizationQuery) assigned(account basics.Address, role keys.Role, targets *disclosure.Record) bool {
	switch role {
	case keys.RecipientView, keys.RecipientSpend:
		return true
	case keys.ManagerAuthority:
		return account == context.params.Manager
	case keys.FreezeAuthority:
		return account == context.params.Freeze
	case keys.ClawbackAuthority:
		return account == context.params.Clawback
	case keys.AuditorView:
		return targets != nil && account == targets.Auditor
	case keys.ControllerView:
		return targets != nil && account == targets.Controller
	}
	return false
}
