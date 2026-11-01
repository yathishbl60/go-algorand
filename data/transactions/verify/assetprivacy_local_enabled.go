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

package verify

import (
	"errors"

	"github.com/algorand/go-algorand/internal/assetprivacy/proofbridge"
)

// IsLocalNativePrivacyFailure identifies local verifier availability/resource
// failures through wrappers. It never treats rejected proofs or limits as local.
func IsLocalNativePrivacyFailure(err error) bool {
	return errors.Is(err, proofbridge.ErrResource) || errors.Is(err, proofbridge.ErrUnavailable)
}
