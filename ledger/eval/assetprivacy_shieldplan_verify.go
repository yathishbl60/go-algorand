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
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"
)

// Bundle requires the main proof and every current required output role.
func (p *AssetPrivacyShieldPlan) Bundle() (*proofbundle.Plan, error) {
	if p == nil {
		return nil, proofbundle.ErrPlan
	}
	return proofbundle.Shield(p.main, p.outputs)
}

// Verify calls the real bridge with this immutable current native statement.
// It does not authorize fees, public conversion or state application.
func (p *AssetPrivacyShieldPlan) Verify(verifier *proofbridge.Verifier, framed []byte) error {
	bundle, err := p.Bundle()
	if err != nil {
		return err
	}
	return verifier.Verify(bundle, privacyBridgeContext(p.Context()), framed)
}
