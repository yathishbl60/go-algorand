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

package recoveryplan

func (k Kind) outputCount() int {
	switch k {
	case PartialPrivate:
		return 2
	case FullPrivate, PartialPublic:
		return 1
	case FullPublic:
		return 0
	default:
		return -1
	}
}

func (k Kind) public() bool { return k == PartialPublic || k == FullPublic }

// RemainderIndex selects only the original-owner remainder of partial recovery.
// It grants no permission to use the weaker retained-recipient output relation.
func (p *Plan) RemainderIndex() (int, bool) {
	if p == nil {
		return 0, false
	}
	switch p.kind {
	case PartialPrivate:
		return 1, true
	case PartialPublic:
		return 0, true
	default:
		return 0, false
	}
}
