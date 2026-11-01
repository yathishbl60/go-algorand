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

func (NativePrivacyScanner) String() string   { return "[native privacy scanner]" }
func (NativePrivacyScanner) GoString() string { return "[native privacy scanner]" }
func (NativePrivacyScanner) MarshalJSON() ([]byte, error) {
	return []byte(`"[native privacy scanner]"`), nil
}
func (NativePrivacyScanner) MarshalText() ([]byte, error) {
	return []byte("[native privacy scanner]"), nil
}
func (NativePrivacyScanCheckpoint) String() string   { return "[native privacy scan checkpoint]" }
func (NativePrivacyScanCheckpoint) GoString() string { return "[native privacy scan checkpoint]" }

// MarshalJSON prevents persisting a checkpoint with redacted, unrecoverable notes.
// Export openings explicitly into authenticated encrypted storage instead.
func (NativePrivacyScanPage) MarshalJSON() ([]byte, error) {
	return nil, ErrNativePrivacyScanStorage
}

func (NativePrivacyScanPage) String() string   { return "[private native privacy scan page]" }
func (NativePrivacyScanPage) GoString() string { return "[private native privacy scan page]" }
func (NativePrivacyScanPage) MarshalText() ([]byte, error) {
	return []byte("[private native privacy scan page]"), nil
}
