//go:build algorand_native_privacy && algorand_native_privacy_stark

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

package assetprivacy

import (
	"encoding/binary"
	"github.com/algorand/go-algorand/internal/assetprivacy/proofbundle"
	"github.com/algorand/go-algorand/internal/assetprivacy/registrationplan"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
	"github.com/algorand/go-algorand/internal/assetprivacy/witness"
	"github.com/stretchr/testify/require"
	"os"
	"path/filepath"
)

func (f *ownerFixture) proofWitnesses(group [64]byte) ([]byte, []byte) {
	context := append([]byte(nil), f.network[:]...)
	context = binary.LittleEndian.AppendUint64(context, uint64(f.asset))
	context = append(context, f.policy[:]...)
	context = append(context, group[:]...)
	filename := filepath.Join(f.artifacts, "public-context.bin")
	require.NoError(f.t, os.WriteFile(filename, context, 0600))
	f.command("prove", f.artifacts, filename)
	require.Equal(f.t, f.owner[:], f.readArtifact("owner.bin"))
	return f.frameProof(group, f.readArtifact("proof.bin")), f.frameProof(group, f.readArtifact("bad-proof.bin"))
}

func (f *ownerFixture) frameProof(group [64]byte, proof []byte) []byte {
	domain := tree.Domain{Genesis: f.network, Asset: uint64(f.asset), Policy: f.policy}
	main, err := registrationplan.New(domain, group, f.owner)
	require.NoError(f.t, err)
	plan, err := proofbundle.RecipientSpendRegistration(main)
	require.NoError(f.t, err)
	limits := proofbundle.Limits{MaxBundleBytes: f.suite.MaxBundleBytes,
		MaxProofBytes: f.suite.MaxProofBytes, MaxAllocationBytes: f.suite.MaxAllocationBytes}
	bundle, err := proofbundle.Encode(plan, [][]byte{proof}, limits)
	require.NoError(f.t, err)
	framed, err := witness.Encode(bundle, witness.Binding{Profile: f.suite.ID, Statement: group},
		witness.Limits{MaxProofBytes: f.suite.MaxBundleBytes})
	require.NoError(f.t, err)
	return framed
}
