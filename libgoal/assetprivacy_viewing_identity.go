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

import (
	"crypto/sha3"
	"encoding/binary"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
)

// The identity binds the public registration role, epoch and encapsulation key.
// It is local scanner metadata and is not sent to the node.
func nativePrivacyViewingIdentity(role apiclient.NativePrivacyRole, epoch uint64, public []byte) [64]byte {
	encoded := append([]byte("Algorand/AssetPrivacy/ScannerViewingKey/V1/"), byte(role))
	encoded = binary.BigEndian.AppendUint64(encoded, epoch)
	return sha3.Sum512(append(encoded, public...))
}
