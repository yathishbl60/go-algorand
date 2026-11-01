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

package v2

import (
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/accounting"
	"github.com/algorand/go-algorand/internal/assetprivacy/closure"
	"github.com/algorand/go-algorand/internal/assetprivacy/inventory"
	"github.com/algorand/go-algorand/internal/assetprivacy/notetree"
	"github.com/algorand/go-algorand/internal/assetprivacy/outputs"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/algorand/go-algorand/internal/assetprivacy/retirement"
	"github.com/algorand/go-algorand/internal/assetprivacy/tree"
)

func nativePrivacyCorruptState(err error) bool {
	for _, target := range []error{query.ErrState, query.ErrConfig, query.ErrReadOnly,
		accounting.ErrEncoding, accounting.ErrSupply, closure.ErrRecord, retirement.ErrRecord,
		inventory.ErrIndex, inventory.ErrMissing, policy.ErrRecord, policy.ErrModes,
		policy.ErrCapabilities, policy.ErrDisclosures, policy.ErrRecoveryDisclosure} {
		if errors.Is(err, target) {
			return true
		}
	}
	return nativePrivacyCorruptTree(err)
}

func nativePrivacyCorruptTree(err error) bool {
	for _, target := range []error{notetree.ErrGeneration, notetree.ErrEncoding, outputs.ErrState,
		outputs.ErrEncoding, outputs.ErrDomain, outputs.ErrDisclosures, outputs.ErrStale,
		tree.ErrEncoding, tree.ErrDomain, tree.ErrCommitment, tree.ErrMissing, tree.ErrHistory, tree.ErrStale} {
		if errors.Is(err, target) {
			return true
		}
	}
	return false
}
