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

// Package accounting checks public aggregate private-supply arithmetic.
// It never receives private note values or verifies spending authorization.
package accounting

// Supply is a current asset's fixed total and public aggregate private supply.
type Supply struct {
	Total   uint64
	Private uint64
}

// Validate rejects a private balance above the asset's fixed total.
func (s Supply) Validate() error {
	if s.Private > s.Total {
		return ErrSupply
	}
	return nil
}

// Shield calculates the private supply after an equal public debit.
// The caller must atomically perform that debit and verify the privacy proof.
func (s Supply) Shield(amount uint64) (Supply, error) {
	if err := s.Validate(); err != nil {
		return s, err
	}
	if amount > s.Total-s.Private {
		return s, ErrPublicSupply
	}
	s.Private += amount
	return s, nil
}

// Unshield calculates the private supply after an equal public credit.
// The caller must atomically perform that credit and verify the privacy proof.
func (s Supply) Unshield(amount uint64) (Supply, error) {
	if err := s.Validate(); err != nil {
		return s, err
	}
	if amount > s.Private {
		return s, ErrPrivateSupply
	}
	s.Private -= amount
	return s, nil
}

// CheckDestruction requires all issued value in the creator's public holding.
func (s Supply) CheckDestruction(creatorHolding uint64) error {
	if err := s.Validate(); err != nil {
		return err
	}
	if s.Private != 0 {
		return ErrOutstanding
	}
	if creatorHolding != s.Total {
		return ErrCreatorHolding
	}
	return nil
}
