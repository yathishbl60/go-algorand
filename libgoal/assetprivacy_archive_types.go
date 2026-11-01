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

import "errors"

var ErrNativePrivacyArchiveKey = errors.New("invalid native privacy archive encryption key")
var ErrNativePrivacyArchive = errors.New("invalid or unauthenticated native privacy scan archive")
var ErrNativePrivacyArchiveDurability = errors.New("native privacy archive published; directory durability is unconfirmed")

const nativePrivacyArchiveMagic = "NAPSC\x01"
const nativePrivacyArchiveCheckpointSize = 258
const nativePrivacyArchiveRecordSize = 629
const nativePrivacyArchivePlainHeaderSize = 259
const nativePrivacyArchivePlainSize = nativePrivacyArchivePlainHeaderSize + 64*nativePrivacyArchiveRecordSize

// NativePrivacyScanArchiveMaxSize is also the exact padded V1 archive file size.
const NativePrivacyScanArchiveMaxSize = len(nativePrivacyArchiveMagic) + 28 + nativePrivacyArchivePlainSize
const nativePrivacyArchiveMinSize = NativePrivacyScanArchiveMaxSize

// NativePrivacyArchiveKey is a uniform local AES-256 key, never an RPC input.
// Its copies and cipher schedules remain in Go-managed memory.
type NativePrivacyArchiveKey struct{ material [32]byte }
