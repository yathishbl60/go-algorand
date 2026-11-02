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
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
	"github.com/stretchr/testify/require"
	"net/http"
	"time"
)

func (f *ownerFixture) rejectPrivatePolicy() {
	queries, err := f.fixture.LibGoalClient.NativePrivacyQueries(&http.Client{Timeout: 30 * time.Second})
	require.NoError(f.t, err)
	before, err := queries.Snapshot(context.Background(), f.asset, nil)
	require.NoError(f.t, err)
	require.Nil(f.t, before.Policy)
	body := lifecyclebody.Body{Operation: lifecyclebody.Adopt, Asset: uint64(f.asset),
		Suite: f.suite.ID, Modes: policy.Modes(policy.Public | policy.Shielded)}
	_, err = f.fixture.LibGoalClient.BroadcastTransaction(f.sign(f.lifecycleTransaction(body)))
	require.ErrorContains(f.t, err, suitecatalog.ErrScope.Error())
	after, err := queries.Snapshot(context.Background(), f.asset, nil)
	require.NoError(f.t, err)
	f.checkRejectedPolicyState(before, after)
	f.t.Log("partial proof scope rejected a private-mode policy without publishing policy or changing public holdings")
}

func (f *ownerFixture) checkRejectedPolicyState(before, after model.NativeAssetPrivacySnapshot) {
	require.True(f.t, after.Exists)
	require.Nil(f.t, after.Policy)
	require.NotNil(f.t, before.CreatorHolding)
	require.EqualValues(f.t, 100, *before.CreatorHolding)
	require.Equal(f.t, before.CreatorHolding, after.CreatorHolding)
	require.Equal(f.t, before.Total, after.Total)
	require.Equal(f.t, before.PrivateSupply, after.PrivateSupply)
	require.Zero(f.t, after.PrivateSupply)
}
