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

package recoveryplan

import "github.com/algorand/go-algorand/internal/assetprivacy/outputs"

// CheckOutputs structurally matches canonical ordered records; it verifies no proof.
// Finalized body/nonce/current-round/authorization correspondence remains native work.
func (p *Plan) CheckOutputs(records []outputs.Record) error {
	if p == nil || p.kind.outputCount() < 0 || len(records) != len(p.outputs) {
		return ErrOutputs
	}
	for index, record := range records {
		if record.Domain != p.domain || record.Output != uint32(index) || record.Commitment != p.outputs[index] {
			return ErrOutputs
		}
		if err := record.Validate(); err != nil {
			return err
		}
	}
	return nil
}
