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

package suitecatalog

import (
	"errors"
	"testing"

	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/internal/assetprivacy/artifactregistry"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func TestScopedRegistryCapabilityPolicyIntegration(t *testing.T) {
	encoded, record := scopeFixture(t, 1<<artifactregistry.OwnerRegistration)
	suite := scopeSelection(t, record)
	registry, err := artifactregistry.Load(encoded, suite.Pin())
	if err != nil || suite.checkRegistry(encoded) != nil {
		t.Fatalf("matching pinned scope rejected: %v", err)
	}
	if registry.ProofKinds() != suite.ProofKinds() {
		t.Fatal("registry/catalog mask disagreement")
	}
	if _, exists := registry.Entry(artifactregistry.Transfer); exists {
		t.Fatal("absent physical slot became a capability")
	}
	checkScopePolicies(t, suite, false)
	record.ProofKinds = artifactregistry.AllProofKinds
	full := scopeSelection(t, record)
	checkScopePolicies(t, full, true)
	checkScopeMismatch(t, full, encoded)
}

func checkScopeMismatch(t *testing.T, suite Suite, encoded []byte) {
	t.Helper()
	if _, err := suite.Verifier(encoded); !errors.Is(err, ErrScope) {
		t.Fatalf("mismatched scope reached backend: %v", err)
	}
}

func checkScopePolicies(t *testing.T, suite Suite, full bool) {
	t.Helper()
	for modes := policy.Modes(1); modes <= policy.AllModes; modes++ {
		err := suite.CheckPolicy(policy.Policy{Modes: modes})
		if full || modes == policy.Modes(policy.Public) {
			if err != nil {
				t.Fatalf("allowed mode %v rejected: %v", modes, err)
			}
		} else if !errors.Is(err, ErrScope) {
			t.Fatalf("incomplete scope accepted private mode %v: %v", modes, err)
		}
	}
	if err := suite.CheckPolicy(policy.Policy{}); !errors.Is(err, policy.ErrModes) {
		t.Fatalf("empty policy accepted: %v", err)
	}
}

func TestScopedCatalogRejectsReservedOrEmptyKinds(t *testing.T) {
	for _, kinds := range []uint32{0, 1, 1 << 15, 1 << 16, 1 << 23, ^uint32(0)} {
		_, record := scopeFixture(t, kinds)
		proto := config.ConsensusParams{EnableNativeAssetPrivacy: true,
			NativeAssetPrivacySuites: []config.NativeAssetPrivacySuite{record}}
		if _, err := Lookup(proto, record.ID); !errors.Is(err, ErrCatalog) {
			t.Fatalf("invalid proof mask %x accepted: %v", kinds, err)
		}
	}
}
