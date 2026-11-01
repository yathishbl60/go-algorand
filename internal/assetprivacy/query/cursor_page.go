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

func (p OutputPage) NextCursor() (string, error) {
	cursor := Cursor{Round: p.Round, Asset: p.Asset, PolicyID: p.PolicyID, Root: p.Root, Count: p.Count, Next: p.Next}
	encoded, err := cursor.Encode()
	if err != nil {
		return "", err
	}
	if !p.HasMore() {
		return "", nil
	}
	return encoded, nil
}

// CheckSnapshot authenticates no cursor; it matches public fields to ledger state.
func (c Cursor) CheckSnapshot(snapshot Snapshot, root [64]byte, count uint64) error {
	if err := c.validate(); err != nil {
		return err
	}
	if c.Round != snapshot.Round || c.Asset != snapshot.Asset || c.PolicyID != snapshot.PolicyID ||
		c.Root != root || c.Count != count {
		return ErrCursor
	}
	return nil
}

func (p OutputPage) HasMore() bool { return p.Next < p.Count }
