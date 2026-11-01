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
	"github.com/spf13/cobra"
)

func init() {
	parent := &cobra.Command{Use: "privacy", Short: "Read native asset privacy state as JSON", Args: cobra.NoArgs}
	for _, name := range []string{"snapshot", "authorization", "outputs", "witness", "key", "eligibility"} {
		parent.AddCommand(nativePrivacyCLICommand(name))
	}
	assetCmd.AddCommand(parent)
}

func nativePrivacyCLICommand(name string) *cobra.Command {
	options := new(nativePrivacyCLIOptions)
	command := &cobra.Command{Use: name, Short: "Read native privacy " + name + " as JSON", Args: cobra.NoArgs}
	nativePrivacyCLIFlags(command, options)
	nativePrivacyCLISpecificFlags(command, options)
	command.RunE = func(cmd *cobra.Command, _ []string) error {
		return nativePrivacyCLIRun(cmd, options)
	}
	return command
}
