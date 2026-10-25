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

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/envelope"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
)

// Build snapshots exact expected vectors from a finalized output and trusted state.
// The future verifier must consume these vectors, not values supplied with a proof.
func Build(record outputs.Record, config Config) (*Plan, error) {
	if err := config.checkOutput(record); err != nil {
		return nil, err
	}
	plan := &Plan{}
	for _, role := range config.roles() {
		slot, err := buildSlot(record, config, role)
		if err != nil {
			return nil, err
		}
		plan.slots = append(plan.slots, slot)
	}
	return plan, nil
}

func buildSlot(r outputs.Record, c Config, role envelope.Role) (Slot, error) {
	context := envelope.Context{Network: r.Domain.Genesis, Asset: r.Domain.Asset,
		Policy: r.Domain.Policy, Role: role, Nonce: r.Nonce, Output: r.Output,
		Commitment: r.Commitment}
	public, err := context.PublicBytes()
	if err != nil {
		return Slot{}, err
	}
	wire, authority, kind := selectRole(r, c, role)
	values := envelopeValues(public, wire, c.KeyRoot.Digest)
	if kind == DisclosureProof {
		values = appendBytes(values, authority[:])
	} else {
		values = appendLimbs(values, c.EligibilityRoot.Digest[:])
	}
	return Slot{role: role, kind: kind, values: values}, nil
}

func selectRole(r outputs.Record, c Config, role envelope.Role) ([]byte, [32]byte, Kind) {
	switch role {
	case envelope.Auditor:
		return r.Auditor, c.Auditor, DisclosureProof
	case envelope.Controller:
		return r.Controller, c.Controller, DisclosureProof
	default: // Build supplies only the validated fixed roles.
		return r.Recipient, [32]byte{}, RecipientProof
	}
}
