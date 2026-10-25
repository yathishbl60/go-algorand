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
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func (c Config) validate() error {
	if c.Domain.Asset == 0 || c.Domain.Genesis == [32]byte{} || c.Domain.Policy == [64]byte{} {
		return ErrConfig
	}
	if c.KeyRoot.Digest == [64]byte{} || c.Disclosures & ^(policy.AuditorDisclosure|policy.ControllerDisclosure) != 0 {
		return ErrConfig
	}
	if (c.Disclosures&policy.AuditorDisclosure != 0) != (c.Auditor != [32]byte{}) {
		return ErrConfig
	}
	if (c.Disclosures&policy.ControllerDisclosure != 0) != (c.Controller != [32]byte{}) {
		return ErrConfig
	}
	return nil
}

func (c Config) checkOutput(record outputs.Record) error {
	if err := c.validate(); err != nil {
		return err
	}
	if record.Domain != c.Domain || record.Disclosures != c.Disclosures {
		return ErrOutput
	}
	return record.Validate()
}

func (c Config) roles() []envelope.Role {
	roles := []envelope.Role{envelope.Recipient}
	if c.Disclosures&policy.AuditorDisclosure != 0 {
		roles = append(roles, envelope.Auditor)
	}
	if c.Disclosures&policy.ControllerDisclosure != 0 {
		roles = append(roles, envelope.Controller)
	}
	return roles
}
