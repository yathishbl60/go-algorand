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

// Read reads fixed canonical records. The caller must hold a consistent round.
func Read(reader Reader, config Config) (Snapshot, error) {
	if reader == nil {
		return Snapshot{}, ErrConfig
	}
	if err := config.validate(); err != nil {
		return Snapshot{}, err
	}
	value := state{Snapshot: Snapshot{Round: config.Round, Asset: config.Asset,
		Creator: config.Creator, Manager: config.Manager, Total: config.Total, Exists: config.Exists,
		CreatorHolding: config.CreatorHolding, HasCreatorHolding: config.HasCreatorHolding}}
	for _, read := range []func(Reader, Config, *state) error{
		readPolicy, readSupply, readRetirement, readClosure, readInventory} {
		if err := read(reader, config, &value); err != nil {
			return Snapshot{}, err
		}
	}
	return value.finish(config)
}

func (s state) finish(config Config) (Snapshot, error) {
	if err := s.validate(config); err != nil {
		return Snapshot{}, err
	}
	if !config.Exists {
		s.Creator, s.Manager, s.Total = s.Closure.Creator, s.Closure.Manager, s.Closure.Total
		s.PolicyID = s.Closure.Policy
	}
	return s.Snapshot, nil
}
