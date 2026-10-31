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

package transactions

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/internal/assetprivacy/lifecyclebody"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/protocol"
)

// IsNativeAssetPrivacyCreation identifies a creation carrying a declaration.
// The predicate grants no admission; CheckNativeAssetPrivacyCreation must follow.
func (tx Transaction) IsNativeAssetPrivacyCreation() bool {
	return tx.Type == protocol.AssetConfigTx && tx.ConfigAsset == 0 && len(tx.NativeAssetPrivacy) != 0
}

// CheckNativeAssetPrivacyCreation checks the exact declaration and ASA fields.
// Actual current PQ account authorization and atomic application remain separate.
func (tx Transaction) CheckNativeAssetPrivacyCreation(proto config.ConsensusParams) error {
	if !tx.IsNativeAssetPrivacyCreation() {
		return lifecyclebody.ErrOperation
	}
	if !proto.Asset || !proto.NativeAssetPrivacyExecutionConfigured() {
		return policy.ErrDisabled
	}
	body, err := lifecyclebody.Decode(tx.NativeAssetPrivacy)
	if err != nil {
		return err
	}
	if body.Operation != lifecyclebody.Create {
		return lifecyclebody.ErrOperation
	}
	if err = tx.AssetConfigTxnFields.wellFormed(proto); err != nil {
		return err
	}
	return tx.checkNativeCreationFields(body)
}

func (tx Transaction) checkNativeCreationFields(body lifecyclebody.Body) error {
	capabilities := policy.Capabilities(0)
	if !tx.AssetParams.Freeze.IsZero() {
		capabilities |= policy.CanFreeze
	}
	if !tx.AssetParams.Clawback.IsZero() {
		capabilities |= policy.CanClawback
	}
	declared := policy.Policy{Modes: body.Modes, Capabilities: body.Capabilities, Disclosures: body.Disclosures}
	if err := policy.CheckCreation(true, declared, capabilities); err != nil {
		return err
	}
	if body.Disclosures&policy.ControllerDisclosure != 0 && tx.AssetParams.Clawback.IsZero() {
		return policy.ErrRecoveryDisclosure
	}
	return body.CheckSender([32]byte(tx.Sender))
}
