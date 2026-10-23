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

package policy

// ExistingAsset contains facts read from the current asset and privacy records.
// Its booleans do not establish authentication; the ledger must supply them.
type ExistingAsset struct {
	Managed      bool
	HasPolicy    bool
	Capabilities Capabilities
}

// CheckCreation matches policy powers to the new asset's authority declarations.
// enabled must come from consensus rules, never from an operator-local toggle.
func CheckCreation(enabled bool, proposed Policy, capabilities Capabilities) error {
	if !enabled {
		return ErrDisabled
	}
	if err := proposed.Validate(); err != nil {
		return err
	}
	if proposed.Capabilities != capabilities {
		return ErrCapabilities
	}
	return nil
}

// CheckAdoption preserves legacy public holdings and existing authority powers.
// The caller must separately verify the manager's required authorizations.
func CheckAdoption(enabled bool, proposed Policy, existing ExistingAsset) error {
	if err := CheckCreation(enabled, proposed, existing.Capabilities); err != nil {
		return err
	}
	if existing.HasPolicy {
		return ErrAlreadyAdopted
	}
	if !existing.Managed {
		return ErrUnmanaged
	}
	if !proposed.Modes.Allows(Public) {
		return ErrPublicRequired
	}
	return nil
}
