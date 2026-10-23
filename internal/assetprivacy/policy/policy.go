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

// Package policy checks the privacy policy's non-cryptographic lifecycle rules.
// It does not authenticate transactions, verify proofs, or mutate ledger state.
package policy

// Capabilities describes currently retained issuer powers.
type Capabilities uint8

const (
	// CanFreeze permits the declared freeze authority.
	CanFreeze Capabilities = 1 << iota
	// CanClawback permits the declared recovery authority.
	CanClawback
)

// Disclosures describes immutable mandatory access to private transfer data.
type Disclosures uint8

const (
	// AuditorDisclosure requires proof-bound auditor ciphertexts.
	AuditorDisclosure Disclosures = 1 << iota
	// ControllerDisclosure declares the recovery information required by clawback.
	ControllerDisclosure
)

// Policy contains semantic fields only; no serialized representation is defined.
type Policy struct {
	Modes        Modes
	Capabilities Capabilities
	Disclosures  Disclosures
}

// Validate rejects empty/unknown modes, unknown flags and undeclared recovery.
func (p Policy) Validate() error {
	if p.Modes == 0 || p.Modes&^AllModes != 0 {
		return ErrModes
	}
	if p.Capabilities & ^(CanFreeze|CanClawback) != 0 {
		return ErrCapabilities
	}
	if p.Disclosures & ^(AuditorDisclosure|ControllerDisclosure) != 0 {
		return ErrDisclosures
	}
	if p.Capabilities&CanClawback != 0 && p.Disclosures&ControllerDisclosure == 0 {
		return ErrRecoveryDisclosure
	}
	return nil
}
