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

package eval

import (
	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/registrationproof"
	"github.com/algorand/go-algorand/protocol"
)

// Current account PQ consent and the new key's possession are separate checks.
func (cs *roundCowState) registerAssetPrivacyLifecycle(body lifecyclebody.Body, binding [64]byte, framed []byte) error {
	if !cs.proto.PQSchemeEnabled(protocol.PQSchemeFalcon1024) {
		return crypto.ErrPQSchemeNotEnabled
	}
	record, err := cs.prepareAssetPrivacyRegistration(body)
	if err != nil {
		return err
	}
	if err = registrationproof.Check(framed); err != nil {
		return err
	}
	if err = cs.reserveAssetPrivacyProofChecks(1); err != nil {
		return err
	}
	if err = registrationproof.Verify(record.PublicKey, binding, framed); err != nil {
		return err
	}
	return cs.PutAssetPrivacyKey(record)
}
