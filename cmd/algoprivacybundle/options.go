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
	"flag"
	"fmt"
	"io"
)

type options struct {
	directory string
	profileID string
}

func parseOptions(args []string, stderr io.Writer) (options, error) {
	var opts options
	flags := flag.NewFlagSet("algoprivacybundle", flag.ContinueOnError)
	flags.SetOutput(stderr)
	flags.StringVar(&opts.directory, "directory", "", "existing directory containing evidence files")
	flags.StringVar(&opts.profileID, "id", "", "review profile identifier (1-128 bytes)")
	if err := flags.Parse(args); err != nil {
		return options{}, err
	}
	if flags.NArg() != 0 || opts.directory == "" || opts.profileID == "" {
		return options{}, fmt.Errorf("usage: algoprivacybundle -directory PATH -id ID (hashes evidence only)")
	}
	return opts, nil
}
