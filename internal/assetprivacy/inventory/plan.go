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

package inventory

import "strings"

// Plan returns owned inventory writes only. The ledger must meter and atomically
// apply them with the actual data mutation; these arguments confer no authority.
func Plan(kv Getter, asset uint64, change Change) ([]Write, error) {
	if err := validateKey(asset, change.Key); err != nil || kv == nil {
		return nil, ErrChange
	}
	if !change.Existed && !change.Present {
		return nil, ErrChange
	}
	m := mutation{kv: kv, asset: asset, change: change, id: identify(change.Key)}
	if err := m.load(); err != nil {
		return nil, err
	}
	if err := m.apply(); err != nil {
		return nil, err
	}
	key, _ := MetadataKey(asset)
	m.writes = append(m.writes, Write{key, encodeMetadata(m.record)})
	return m.writes, m.record.Validate()
}

func (m *mutation) load() error {
	record, exists, err := Read(m.kv, m.asset)
	if err != nil {
		return err
	}
	if !exists && (!m.change.Initialize || m.change.Existed || !strings.HasPrefix(m.change.Key, "aprv:p:")) {
		return ErrMissing
	}
	m.record = record
	value, found, err := readNode(m.kv, m.asset, m.id)
	if err != nil {
		return err
	}
	if found != m.change.Existed || (found && (value.Key != m.change.Key || value.ValueBytes != m.change.Before)) {
		return ErrIndex
	}
	m.node = value
	return nil
}

func (m *mutation) apply() error {
	if !m.change.Existed {
		return m.insert()
	}
	if err := m.checkPosition(); err != nil {
		return err
	}
	if !m.change.Present {
		return m.remove()
	}
	if err := m.replaceBytes(); err != nil {
		return err
	}
	m.node.ValueBytes = m.change.After
	m.stage(m.id, m.node)
	return nil
}

func (m *mutation) stage(id ID, value node) {
	m.writes = append(m.writes, Write{nodeKey(m.asset, id), encodeNode(value)})
}
