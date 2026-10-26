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

func (r *Registry) Profile() [64]byte          { return r.pin.Profile }
func (r *Registry) Implementation() [64]byte   { return r.pin.Implementation }
func (r *Registry) Digest() [64]byte           { return r.pin.Digest }
func (r *Registry) Entries() [EntryCount]Entry { return r.entries }

// Entry returns a value copy for a supported native layout; it never selects a proof.
func (r *Registry) Entry(kind Kind) (Entry, bool) {
	if r == nil || kind == 0 {
		return Entry{}, false
	}
	for _, entry := range r.entries {
		if entry.Kind == kind {
			return entry, true
		}
	}
	return Entry{}, false
}
