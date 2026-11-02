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
	"encoding/hex"
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/bookkeeping"
	"github.com/algorand/go-algorand/protocol"
	"github.com/stretchr/testify/require"
	"os"
	"path/filepath"
	"time"
)

func (f *ownerFixture) setupNetwork() {
	f.fixture.AlterConsensus(protocol.ConsensusFuture, f.ownerConsensus)
	f.fixture.SetupNoStart(f.t, filepath.Join("nettemplates", "TwoNodes50EachFuture.json"))
	for _, name := range []string{"Primary", "Node"} {
		directory, err := f.fixture.GetNodeDir(name)
		require.NoError(f.t, err)
		f.installRegistry(directory)
	}
}

func (f *ownerFixture) ownerConsensus(p config.ConsensusParams) config.ConsensusParams {
	require.False(f.t, config.Consensus[protocol.ConsensusFuture].EnableNativeAssetPrivacy)
	p.EnableNativeAssetPrivacy = true
	p.NativeAssetPrivacyRootHistory = 64
	p.NativeAssetPrivacyFlatMinBalance, p.NativeAssetPrivacyByteMinBalance = 1000, 10
	p.NativeAssetPrivacyMaxProofChecksPerBlock = 4
	p.NativeAssetPrivacyProofFeeFactor, p.NativeAssetPrivacyByteFeeFactor = 1_000_000, 1
	p.NativeAssetPrivacySuites = []config.NativeAssetPrivacySuite{f.suite}
	p.AgreementFilterTimeoutPeriod0, p.AgreementFilterTimeout = 400*time.Millisecond, 400*time.Millisecond
	require.True(f.t, p.NativeAssetPrivacyExecutionConfigured())
	require.True(f.t, p.EnablePQSchemeFalcon1024)
	return p
}

func (f *ownerFixture) installRegistry(directory string) {
	genesis, err := bookkeeping.LoadGenesisFromFile(filepath.Join(directory, "genesis.json"))
	require.NoError(f.t, err)
	root := filepath.Join(directory, genesis.ID(), "native-privacy", "registries")
	require.NoError(f.t, os.MkdirAll(root, 0700))
	filename := filepath.Join(root, hex.EncodeToString(f.suite.RegistryDigest[:])+".napk")
	require.NoError(f.t, os.WriteFile(filename, f.registry, 0600))
}
