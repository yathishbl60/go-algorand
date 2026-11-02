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

package artifactregistry

// Every physical slot has its fixed layout; only real keys publish capabilities.
func (e Entry) validScope() bool {
	zero := e.Root == [8]uint64{}
	return e.Flags == 0 && zero || e.Flags == 3 && !zero
}

func anyPresent(entries [EntryCount]Entry) bool {
	for _, entry := range entries {
		if entry.Flags == 3 {
			return true
		}
	}
	return false
}
