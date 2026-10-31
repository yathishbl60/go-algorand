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

func (m *mutation) insert() error {
	value := node{Key: m.change.Key, ValueBytes: m.change.After}
	if m.record.Items != 0 {
		tail, exists, err := readNode(m.kv, m.asset, m.record.Tail)
		if err != nil {
			return err
		}
		if !exists || tail.Next.Exists || m.id == m.record.Tail || m.id == m.record.Head {
			return ErrIndex
		}
		value.Prev = link{true, m.record.Tail}
		tail.Next = link{true, m.id}
		m.stage(m.record.Tail, tail)
	} else {
		m.record.Head = m.id
	}
	m.record.Tail = m.id
	m.stage(m.id, value)
	return m.addItem()
}
