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

package witness

// Error identifies framing failures and supports errors.Is/errors.As.
// Stream I/O failures also wrap their original underlying error.
type Error uint8

const (
	ErrLimits Error = 1 + iota
	ErrBinding
	ErrVersion
	ErrMagic
	ErrLength
	ErrTruncated
	ErrTrailing
	ErrIO
)

func (e Error) Error() string {
	messages := [...]string{"unknown privacy witness error", "invalid trusted witness bounds",
		"privacy witness binding mismatch", "unsupported privacy witness version",
		"invalid privacy witness magic", "invalid or oversized privacy proof length",
		"truncated privacy witness", "trailing privacy witness bytes", "privacy witness I/O failure"}
	if int(e) >= len(messages) {
		return messages[0]
	}
	return messages[e]
}
