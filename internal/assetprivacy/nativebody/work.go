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

package nativebody

import "github.com/algorand/go-algorand/internal/assetprivacy/policy"

// ProofChecks counts one main proof and every mandatory output-role proof.
// Each check costs one conservative worst-profile unit, never measured wall time.
// This coarse count is not evidence of practical node memory or validation budgets.
func (b Body) ProofChecks() (uint64, error) {
	if err := b.Validate(); err != nil {
		return 0, err
	}
	checks := uint64(1)
	for _, output := range b.Outputs {
		checks++
		if output.Disclosures&policy.AuditorDisclosure != 0 {
			checks++
		}
		if output.Disclosures&policy.ControllerDisclosure != 0 {
			checks++
		}
	}
	return checks, nil
}
