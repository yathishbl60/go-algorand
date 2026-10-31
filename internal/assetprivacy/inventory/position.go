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

func (m *mutation) checkPosition() error {
	if m.record.Items == 0 {
		return ErrIndex
	}
	prev, next := m.node.Prev, m.node.Next
	if (!prev.Exists && m.record.Head != m.id) || (!next.Exists && m.record.Tail != m.id) {
		return ErrIndex
	}
	if (prev.Exists && (prev.ID == m.id || m.record.Head == m.id)) || (next.Exists && (next.ID == m.id || m.record.Tail == m.id)) {
		return ErrIndex
	}
	if prev.Exists && next.Exists && prev.ID == next.ID {
		return ErrIndex
	}
	return m.checkCardinality()
}

func (m *mutation) checkCardinality() error {
	if (m.record.Items == 1) != (!m.node.Prev.Exists && !m.node.Next.Exists) {
		return ErrIndex
	}
	if m.record.Items == 1 {
		size, err := footprint(m.node.Key, m.node.ValueBytes)
		if err != nil || size != m.record.Bytes {
			return ErrIndex
		}
	}
	return nil
}
