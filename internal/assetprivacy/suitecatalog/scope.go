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

package suitecatalog

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

// ProofKinds reports independently selected consensus metadata, not file availability.
func (s Suite) ProofKinds() uint32 { return s.record.ProofKinds }

// CheckPolicy forbids private modes until the selected suite supports every native kind.
// Partial suites support public-only policies without creating private inventory.
func (s Suite) CheckPolicy(proposed policy.Policy) error {
	if err := checkRecord(s.record); err != nil {
		return err
	}
	if err := proposed.Validate(); err != nil {
		return err
	}
	if proposed.Modes != policy.Modes(policy.Public) && s.ProofKinds() != artifactregistry.AllProofKinds {
		return ErrScope
	}
	return nil
}

func (s Suite) checkRegistry(encoded []byte) error {
	registry, err := artifactregistry.Load(encoded, s.Pin())
	if err != nil {
		return err
	}
	if registry.ProofKinds() != s.ProofKinds() {
		return ErrScope
	}
	return nil
}
