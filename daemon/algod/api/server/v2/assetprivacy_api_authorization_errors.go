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

	"github.com/algorand/go-algorand/internal/assetprivacy/control"
	"github.com/algorand/go-algorand/internal/assetprivacy/disclosure"
	"github.com/algorand/go-algorand/internal/assetprivacy/eligibility"
	"github.com/algorand/go-algorand/internal/assetprivacy/keyauth"
	"github.com/algorand/go-algorand/internal/assetprivacy/keys"
)

func nativePrivacyCorruptAuthorization(err error) bool {
	for _, target := range []error{keys.ErrDomain, keys.ErrKey, keys.ErrRole, keys.ErrRecord, keys.ErrEpoch, keys.ErrHistory, keys.ErrAuthority,
		keyauth.ErrDomain, keyauth.ErrRecord, keyauth.ErrHistory,
		eligibility.ErrDomain, eligibility.ErrRecord, eligibility.ErrHistory, eligibility.ErrRevision, eligibility.ErrRound,
		control.ErrDomain, control.ErrRecord, control.ErrEpoch, control.ErrHistory, control.ErrStale,
		disclosure.ErrDomain, disclosure.ErrFlags, disclosure.ErrRecord, disclosure.ErrEpoch, disclosure.ErrRound,
		disclosure.ErrMissing, disclosure.ErrTarget, disclosure.ErrStale} {
		if errors.Is(err, target) {
			return true
		}
	}
	return false
}
