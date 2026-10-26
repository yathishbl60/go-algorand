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

type layout struct {
	kind                               Kind
	rows, width, preprocessing, public uint32
}

// Layouts match the compiled NullifierV2 native AIRs and both next-row openings.
func layouts() [EntryCount]layout {
	return [EntryCount]layout{
		{Transfer, 16384, 8506, 80, 264},
		{Unshield, 16384, 8506, 80, 196},
		{PartialPrivate, 8192, 8539, 60, 192},
		{FullPrivate, 8192, 8539, 60, 160},
		{PartialPublic, 8192, 8539, 60, 164},
		{FullPublic, 8192, 8539, 60, 132},
		{Recipient, 65536, 24869, 2075, 1132},
		{Disclosure, 65536, 23973, 2067, 1132},
		{RetainedRecipient, 65536, 24229, 2067, 1100},
		{RecoveryRecipient, 65536, 24869, 2071, 1100},
	}
}

func (e Entry) matches(expected layout) bool {
	return e.Kind == expected.kind && e.Rows == expected.rows && e.MainWidth == expected.width &&
		e.PreWidth == expected.preprocessing && e.PublicCount == expected.public && e.Flags == 3
}
