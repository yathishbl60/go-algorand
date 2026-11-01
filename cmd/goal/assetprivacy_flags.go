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
	"time"

	"github.com/spf13/cobra"
)

func nativePrivacyCLIFlags(cmd *cobra.Command, options *nativePrivacyCLIOptions) {
	flags := cmd.Flags()
	flags.Uint64Var(&options.asset, "assetid", 0, "Asset ID (required, positive)")
	flags.Uint64Var(&options.round, "round", 0, "Pinned round (omitted: latest, explicit zero: round zero)")
	flags.DurationVar(&options.timeout, "timeout", 30*time.Second, "Positive request deadline")
	cmd.MarkFlagRequired("assetid")
}

func nativePrivacyCLISpecificFlags(cmd *cobra.Command, options *nativePrivacyCLIOptions) {
	flags := cmd.Flags()
	switch cmd.Name() {
	case "outputs":
		flags.Uint64Var(&options.start, "start", 0, "First output index (through 2^32)")
		flags.Uint32Var(&options.limit, "limit", 32, "Page size, at most 64 (zero means 32)")
		flags.StringVar(&options.next, "next", "", "Pinned continuation token; excludes --round and --start")
	case "witness":
		flags.Uint32Var(&options.index, "index", 0, "Note index (required)")
		cmd.MarkFlagRequired("index")
	case "key", "eligibility":
		nativePrivacyCLIAccountFlags(cmd, options)
	}
}

func nativePrivacyCLIAccountFlags(cmd *cobra.Command, options *nativePrivacyCLIOptions) {
	cmd.Flags().StringVar(&options.account, "account", "", "Checksummed account address (required, nonzero)")
	cmd.MarkFlagRequired("account")
	if cmd.Name() == "key" {
		cmd.Flags().StringVar(&options.role, "role", "", "Key role: recipient-view, recipient-spend, auditor-view, controller-view, freeze-authority, clawback-authority, manager-authority")
		cmd.MarkFlagRequired("role")
	}
}
