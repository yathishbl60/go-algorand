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

// AllProofKinds has bits at native kind IDs 1..14 and 17..22; other bits are reserved.
const AllProofKinds uint32 = ((1 << 15) - 2) | ((1 << 23) - (1 << 17))

// ProofKinds returns only present capabilities from the independently pinned file.
func (r *Registry) ProofKinds() uint32 {
	var kinds uint32
	if r == nil {
		return 0
	}
	for _, entry := range r.entries {
		if entry.Flags == 3 {
			kinds |= 1 << entry.Kind
		}
	}
	return kinds
}
