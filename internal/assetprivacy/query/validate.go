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

package query

import "github.com/algorand/go-algorand/internal/assetprivacy/accounting"

func (s *state) validate(config Config) error {
	if !config.Exists {
		return s.validateClosed(config)
	}
	if s.Policy == nil {
		return s.validateOrdinary()
	}
	if s.Inventory == nil || s.Inventory.Items == 0 || s.Policy.Created > config.Round {
		return ErrState
	}
	identity, err := s.Policy.Identity(config.Network, config.Asset)
	if err != nil {
		return err
	}
	s.PolicyID = identity
	return s.validateActive(config)
}

func (s state) validateOrdinary() error {
	if s.supplyPresent || s.Retired != nil || s.Closure != nil || s.Inventory != nil {
		return ErrState
	}
	return nil
}

func (s state) validateActive(config Config) error {
	if err := (accounting.Supply{Total: config.Total, Private: s.Private}).Validate(); err != nil {
		return err
	}
	if s.Retired != nil && (s.Retired.Policy != s.PolicyID || s.Retired.Suite != s.Policy.Suite ||
		s.Retired.Manager != config.Manager || s.Retired.Created > config.Round || s.Retired.Created < s.Policy.Created ||
		!config.HasCreatorHolding || config.CreatorHolding != config.Total || s.Private != 0) {
		return ErrState
	}
	if s.Closure != nil {
		return ErrState // A committed round cannot retain a pending destruction pair.
	}
	return nil
}

func (s state) validateClosed(config Config) error {
	if s.Closure == nil {
		if s.Policy != nil || s.supplyPresent || s.Retired != nil || s.Inventory != nil {
			return ErrState
		}
		return ErrNotFound
	}
	if s.Policy != nil || s.supplyPresent || s.Retired != nil || s.Inventory == nil ||
		s.Closure.Created > config.Round || s.Inventory.Items == 0 {
		return ErrState
	}
	return nil
}
