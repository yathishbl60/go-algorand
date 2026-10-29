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
	"errors"
	"github.com/algorand/go-algorand/internal/assetprivacy/nativebody"
)

var errAssetPrivacyWork = errors.New("native privacy proof work budget exceeded or unconfigured")

func (cs *roundCowState) reserveAssetPrivacyProofChecks(checks uint64) error {
	if !cs.proto.NativeAssetPrivacyExecutionConfigured() || checks == 0 || checks > 7 {
		return errAssetPrivacyWork
	}
	remaining := cs.proto.NativeAssetPrivacyMaxProofChecksPerBlock
	for state := cs; state != nil; state = state.commitParent {
		if state.nativePrivacyProofChecks > remaining {
			return errAssetPrivacyWork
		}
		remaining -= state.nativePrivacyProofChecks
	}
	if checks > remaining {
		return errAssetPrivacyWork
	}
	cs.nativePrivacyProofChecks += checks
	return nil
}

func (cs *roundCowState) commitAssetPrivacyProofChecks() {
	if cs.nativePrivacyProofChecks == 0 {
		return
	}
	parent := cs.commitParent
	maximum := cs.proto.NativeAssetPrivacyMaxProofChecksPerBlock
	if parent.nativePrivacyProofChecks > maximum || cs.nativePrivacyProofChecks > maximum-parent.nativePrivacyProofChecks {
		panic(errAssetPrivacyWork)
	}
	parent.nativePrivacyProofChecks += cs.nativePrivacyProofChecks
}

func (cs *roundCowState) reserveAssetPrivacyBodyWork(body nativebody.Body) error {
	checks, err := body.ProofChecks()
	if err != nil {
		return err
	}
	return cs.reserveAssetPrivacyProofChecks(checks)
}
