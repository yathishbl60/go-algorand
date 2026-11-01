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

package main

import (
	"errors"

	"github.com/spf13/cobra"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/data/basics"
)

func nativePrivacyCLIValidate(cmd *cobra.Command, options *nativePrivacyCLIOptions) error {
	if options.asset == 0 || options.timeout <= 0 {
		return errors.New("--assetid and --timeout must be positive")
	}
	if cmd.Name() == "outputs" {
		if options.start > 1<<32 || options.limit > 64 {
			return errors.New("--start must be at most 2^32; --limit must be at most 64")
		}
		if cmd.Flags().Changed("next") && (options.next == "" || cmd.Flags().Changed("start") || cmd.Flags().Changed("round")) {
			return errors.New("--next must be nonempty and cannot be combined with --start or --round")
		}
	}
	if cmd.Name() == "key" || cmd.Name() == "eligibility" {
		return nativePrivacyCLIValidateAccount(cmd, options)
	}
	return nil
}

func nativePrivacyCLIValidateAccount(cmd *cobra.Command, options *nativePrivacyCLIOptions) error {
	address, err := basics.UnmarshalChecksumAddress(options.account)
	if err != nil || address.IsZero() {
		return errors.New("--account must be a nonzero checksummed address")
	}
	options.address = address
	if cmd.Name() == "key" {
		options.keyRole = nativePrivacyCLIRole(options.role)
		if options.keyRole == 0 {
			return errors.New("--role must name a supported native privacy key role")
		}
	}
	return nil
}

func nativePrivacyCLIRole(name string) apiclient.NativePrivacyRole {
	roles := map[string]apiclient.NativePrivacyRole{
		"recipient-view":     apiclient.NativePrivacyRecipientView,
		"recipient-spend":    apiclient.NativePrivacyRecipientSpend,
		"auditor-view":       apiclient.NativePrivacyAuditorView,
		"controller-view":    apiclient.NativePrivacyControllerView,
		"freeze-authority":   apiclient.NativePrivacyFreezeAuthority,
		"clawback-authority": apiclient.NativePrivacyClawbackAuthority,
		"manager-authority":  apiclient.NativePrivacyManagerAuthority,
	}
	return roles[name]
}
