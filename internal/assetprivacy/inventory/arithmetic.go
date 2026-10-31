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

func (m *mutation) replaceBytes() error {
	old, err := footprint(m.change.Key, m.change.Before)
	if err != nil || old > m.record.Bytes {
		return ErrIndex
	}
	next, err := footprint(m.change.Key, m.change.After)
	if err != nil {
		return err
	}
	m.record.Bytes -= old
	if next > ^uint64(0)-m.record.Bytes {
		return ErrIndex
	}
	m.record.Bytes += next
	return nil
}

func footprint(key string, value uint64) (uint64, error) {
	size := uint64(len(key))
	if value > ^uint64(0)-size {
		return 0, ErrIndex
	}
	return size + value, nil
}

func (m *mutation) addItem() error {
	size, err := footprint(m.change.Key, m.change.After)
	if err != nil || m.record.Items == ^uint64(0) || size > ^uint64(0)-m.record.Bytes {
		return ErrIndex
	}
	m.record.Items++
	m.record.Bytes += size
	return nil
}

func (m *mutation) subtractItem() error {
	size, err := footprint(m.change.Key, m.change.Before)
	if err != nil || m.record.Items == 0 || m.record.Bytes < size {
		return ErrIndex
	}
	m.record.Items--
	m.record.Bytes -= size
	return nil
}
