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

package proofbridge

import (
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"
	"github.com/algorand/go-algorand/internal/assetprivacy/witness"
)

// Verify checks the complete expected bundle before calling the optional backend.
// Keep framedNAPW stable until return; decoding makes an owned backend payload copy.
// A nil result establishes cryptographic checking only, not native authorization.
func (v *Verifier) Verify(plan *proofbundle.Plan, context Context, framedNAPW []byte) error {
	if err := v.ready(context); err != nil {
		return err
	}
	bundle, err := v.decode(framedNAPW, context)
	if err != nil {
		return err
	}
	if err = proofbundle.Check(bundle, plan, v.limits.bundle()); err != nil {
		return err
	}
	request, err := v.request(plan, context)
	if err != nil {
		return err
	}
	return verifyBackend(request, bundle)
}

func (v *Verifier) ready(context Context) error {
	if v == nil || v.registry == nil {
		return ErrUnavailable
	}
	if context.Domain.Genesis == [32]byte{} || context.Domain.Asset == 0 || context.Domain.Policy == [64]byte{} ||
		context.Group == [64]byte{} || context.Suite != v.pin.Profile {
		return ErrContext
	}
	return backendStatus(v.pin.Implementation)
}

func (v *Verifier) decode(framed []byte, context Context) ([]byte, error) {
	if len(framed) < witness.HeaderSize {
		return nil, witness.ErrTruncated
	}
	if uint64(len(framed)-witness.HeaderSize) > uint64(v.limits.MaxAllocationBytes) {
		return nil, ErrLimits
	}
	binding := witness.Binding{Profile: context.Suite, Statement: context.Group}
	return witness.Decode(framed, binding, witness.Limits{MaxProofBytes: v.limits.MaxBundleBytes})
}
