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

func parseOptions(args []string, stderr io.Writer) (string, error) {
	flags := flag.NewFlagSet("algoprivacy", flag.ContinueOnError)
	flags.SetOutput(stderr)
	path := flags.String("profile", "", "path to a local privacy evidence manifest")
	if err := flags.Parse(args); err != nil {
		return "", err
	}
	if flags.NArg() != 0 || *path == "" {
		return "", fmt.Errorf("usage: algoprivacy -profile PATH (checks evidence integrity only)")
	}
	return *path, nil
}
