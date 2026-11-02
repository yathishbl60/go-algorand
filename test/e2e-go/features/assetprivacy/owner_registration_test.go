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
	"github.com/algorand/go-algorand/test/framework/fixtures"
	"github.com/algorand/go-algorand/test/partitiontest"
	"testing"
)

// This is native owner-key registration, not a private asset transfer scenario.
func TestNativeOwnerRegistration(t *testing.T) {
	partitiontest.PartitionTest(t)
	defer fixtures.ShutdownSynchronizedTest(t)
	f := newOwnerFixture(t)
	defer f.fixture.Shutdown()
	f.fixture.Start()
	f.fundAccounts()
	f.createAsset()
	f.rejectPrivatePolicy()
	f.adoptPolicy()
	group, invalid := f.registrationGroup()
	f.rejectInvalidGroup(group, invalid)
	f.acceptRegistration(group)
}
