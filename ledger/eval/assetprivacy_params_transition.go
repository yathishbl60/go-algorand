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
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
)

func privacyAssetCapabilities(params basics.AssetParams) policy.Capabilities {
	capabilities := policy.Capabilities(0)
	if !params.Freeze.IsZero() {
		capabilities |= policy.CanFreeze
	}
	if !params.Clawback.IsZero() {
		capabilities |= policy.CanClawback
	}
	return capabilities
}

func checkPrivacyAssetParamsTransition(before, after basics.AssetParams, declaration policy.Record) error {
	if declaration.Policy.Capabilities != privacyAssetCapabilities(before) {
		return policy.ErrCapabilities
	}
	if privacyAuthorityRestored(before, after) {
		return policy.ErrAuthorityExpansion
	}
	normalized := before
	normalized.Manager = after.Manager
	normalized.Reserve = after.Reserve
	normalized.Freeze = after.Freeze
	normalized.Clawback = after.Clawback
	if normalized != after {
		return policy.ErrImmutable
	}
	return nil
}

func privacyAuthorityRestored(before, after basics.AssetParams) bool {
	return (before.Manager.IsZero() && !after.Manager.IsZero()) ||
		(before.Reserve.IsZero() && !after.Reserve.IsZero()) ||
		(before.Freeze.IsZero() && !after.Freeze.IsZero()) ||
		(before.Clawback.IsZero() && !after.Clawback.IsZero())
}
