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

package policy

import "errors"

// Lifecycle errors can be classified with errors.Is; these are not wire codes.
var (
	ErrDisabled           = errors.New("native asset privacy is disabled")
	ErrModes              = errors.New("privacy policy requires a nonempty set of known modes")
	ErrCapabilities       = errors.New("privacy capabilities do not match retained asset authorities")
	ErrDisclosures        = errors.New("unknown privacy disclosure flags")
	ErrRecoveryDisclosure = errors.New("clawback requires declared controller disclosure")
	ErrUnmanaged          = errors.New("an existing asset without a manager cannot adopt privacy")
	ErrAlreadyAdopted     = errors.New("asset privacy policy has already been adopted")
	ErrPublicRequired     = errors.New("existing assets must retain public mode")
	ErrImmutable          = errors.New("privacy modes and disclosures are immutable")
	ErrAuthorityExpansion = errors.New("relinquished privacy capabilities cannot be restored")
)
