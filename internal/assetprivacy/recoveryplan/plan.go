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

package recoveryplan

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/spendplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

// Plan is an immutable expected statement. It neither proves authority nor owns funds.
type Plan struct {
	kind      Kind
	values    []uint16
	domain    tree.Domain
	authority [32]byte
	input     spendplan.Input
	outputs   [][64]byte
}

func (p *Plan) Domain() tree.Domain    { return p.domain }
func (p *Plan) Kind() Kind             { return p.kind }
func (p *Plan) Authority() [32]byte    { return p.authority }
func (p *Plan) Values() []uint16       { return append([]uint16(nil), p.values...) }
func (p *Plan) Input() spendplan.Input { return p.input }
func (p *Plan) Outputs() [][64]byte    { return append([][64]byte(nil), p.outputs...) }

// Build derives exact expected vectors; no amount, ownership or issuer proof is checked.
func Build(statement Statement, config Config) (*Plan, error) {
	if err := config.validate(); err != nil {
		return nil, err
	}
	if err := statement.validate(); err != nil {
		return nil, err
	}
	return &Plan{kind: statement.Kind, values: publicValues(statement, config),
		domain: config.Domain, authority: config.Authority, input: statement.Input,
		outputs: append([][64]byte(nil), statement.Outputs...)}, nil
}
