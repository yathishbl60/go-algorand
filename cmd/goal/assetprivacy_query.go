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
	"context"
	"errors"

	"github.com/spf13/cobra"

	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/data/basics"
)

func nativePrivacyCLIQuery(ctx context.Context, client *apiclient.NativePrivacyClient, cmd *cobra.Command, options *nativePrivacyCLIOptions) (any, error) {
	round := nativePrivacyCLIRound(cmd, options)
	asset := basics.AssetIndex(options.asset)
	switch cmd.Name() {
	case "snapshot":
		return client.Snapshot(ctx, asset, round)
	case "authorization":
		return client.Authorization(ctx, asset, round)
	case "outputs":
		return client.Outputs(ctx, asset, apiclient.NativePrivacyOutputOptions{Round: round, Start: options.start, Limit: options.limit, Next: options.next})
	case "witness":
		return client.NoteWitness(ctx, asset, options.index, round)
	case "key":
		return client.Key(ctx, asset, options.address, options.keyRole, round)
	case "eligibility":
		return client.Eligibility(ctx, asset, options.address, round)
	}
	return nil, errors.New("unsupported native privacy query")
}

func nativePrivacyCLIRound(cmd *cobra.Command, options *nativePrivacyCLIOptions) *basics.Round {
	if !cmd.Flags().Changed("round") {
		return nil
	}
	round := basics.Round(options.round)
	return &round
}
