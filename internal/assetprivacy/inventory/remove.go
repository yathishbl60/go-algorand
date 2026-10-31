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

func (m *mutation) remove() error {
	if err := m.unlinkPrev(); err != nil {
		return err
	}
	if err := m.unlinkNext(); err != nil {
		return err
	}
	if err := m.subtractItem(); err != nil {
		return err
	}
	if m.record.Items == 0 {
		m.record.Head, m.record.Tail = ID{}, ID{}
	}
	m.writes = append(m.writes, Write{Key: nodeKey(m.asset, m.id)})
	return nil
}

func (m *mutation) unlinkPrev() error {
	if !m.node.Prev.Exists {
		m.record.Head = m.node.Next.ID
		return nil
	}
	prev, exists, err := readNode(m.kv, m.asset, m.node.Prev.ID)
	if err != nil {
		return err
	}
	if !exists || prev.Next != (link{true, m.id}) {
		return ErrIndex
	}
	prev.Next = m.node.Next
	m.stage(m.node.Prev.ID, prev)
	return nil
}

func (m *mutation) unlinkNext() error {
	if !m.node.Next.Exists {
		m.record.Tail = m.node.Prev.ID
		return nil
	}
	next, exists, err := readNode(m.kv, m.asset, m.node.Next.ID)
	if err != nil {
		return err
	}
	if !exists || next.Prev != (link{true, m.id}) {
		return ErrIndex
	}
	next.Prev = m.node.Prev
	m.stage(m.node.Next.ID, next)
	return nil
}
