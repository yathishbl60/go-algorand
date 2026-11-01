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

package libgoal

// String prevents ordinary diagnostic formatting from exposing recovered secrets.
func (NativePrivacyRecoveredNote) String() string {
	return "[private recovered asset note]"
}

func (NativePrivacyRecoveredNote) GoString() string {
	return "[private recovered asset note]"
}

func (NativePrivacyRecoveredNote) MarshalJSON() ([]byte, error) {
	return []byte(`"[private recovered asset note]"`), nil
}

func (NativePrivacyRecoveredNote) MarshalText() ([]byte, error) {
	return []byte("[private recovered asset note]"), nil
}
