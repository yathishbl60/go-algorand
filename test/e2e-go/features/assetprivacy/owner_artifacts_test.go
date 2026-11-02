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
	"context"
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
	"github.com/stretchr/testify/require"
	"os"
	"os/exec"
	"path/filepath"
	"testing"
	"time"
)

func newOwnerFixture(t *testing.T) *ownerFixture {
	t.Helper()
	binary := os.Getenv("NAP_OWNER_FIXTURE_BIN")
	require.NotEmpty(t, binary, "build native_owner_fixture and set NAP_OWNER_FIXTURE_BIN")
	f := &ownerFixture{t: t, binary: binary, artifacts: t.TempDir()}
	f.command("setup", f.artifacts)
	f.loadArtifacts()
	f.setupNetwork()
	return f
}

func (f *ownerFixture) command(args ...string) {
	f.t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
	defer cancel()
	output, err := exec.CommandContext(ctx, f.binary, args...).CombinedOutput()
	require.NoError(f.t, err, "native owner fixture failed: %s", output)
}

func (f *ownerFixture) readArtifact(name string) []byte {
	f.t.Helper()
	data, err := os.ReadFile(filepath.Join(f.artifacts, name))
	require.NoError(f.t, err)
	return data
}

func (f *ownerFixture) readDigest(name string) [64]byte {
	data := f.readArtifact(name)
	require.Len(f.t, data, 64)
	return [64]byte(data)
}

func (f *ownerFixture) loadArtifacts() {
	f.registry = f.readArtifact("registry.napk")
	f.suite = config.NativeAssetPrivacySuite{ID: f.readDigest("profile.bin"),
		Reference: f.readDigest("implementation.bin"), RegistryDigest: f.readDigest("digest.bin"),
		ProofKinds: 1 << artifactregistry.OwnerRegistration, MaxBundleBytes: 5_000_000,
		MaxProofBytes: 4_999_000, MaxAllocationBytes: 128 << 20}
	selected, err := suitecatalog.Lookup(config.ConsensusParams{EnableNativeAssetPrivacy: true,
		NativeAssetPrivacySuites: []config.NativeAssetPrivacySuite{f.suite}}, f.suite.ID)
	require.NoError(f.t, err)
	_, err = selected.Verifier(f.registry)
	require.NoError(f.t, err, "real installed registry must match capabilities and backend before startup")
}
