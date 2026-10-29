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

// Called only with a complete proof set checked against this unchanged child.
func (cs *roundCowState) applyVerifiedAssetPrivacy(execution *assetPrivacyExecution) error {
	conversion := execution.plan.conversion
	if err := cs.applyAssetPrivacyPublicConversion(conversion, execution.plan.recovery != nil); err != nil {
		return err
	}
	if err := cs.adjustAssetPrivacyPrivateSupply(conversion); err != nil {
		return err
	}
	for _, input := range execution.body.Inputs {
		if err := cs.ConsumeAssetPrivacyNullifier(conversion.Asset, input.Nullifier); err != nil {
			return err
		}
	}
	for _, output := range execution.records {
		if _, _, err := cs.appendAssetPrivacyOutput(conversion.Asset, output); err != nil {
			return err
		}
	}
	return nil
}

func (cs *roundCowState) adjustAssetPrivacyPrivateSupply(value AssetPrivacyPublicConversion) error {
	if value.Debit != 0 {
		if err := cs.IncreaseAssetPrivateSupply(value.Asset, value.Debit); err != nil {
			return err
		}
	}
	if value.Credit != 0 {
		return cs.DecreaseAssetPrivateSupply(value.Asset, value.Credit)
	}
	return nil
}
