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
	"encoding/json"
	"net/http"
	"time"

	"github.com/spf13/cobra"

	"github.com/algorand/go-algorand/cmd/util/datadir"
	apiclient "github.com/algorand/go-algorand/daemon/algod/api/client"
	"github.com/algorand/go-algorand/libgoal"
)

func nativePrivacyCLIRun(cmd *cobra.Command, options *nativePrivacyCLIOptions) error {
	if err := nativePrivacyCLIValidate(cmd, options); err != nil {
		return err
	}
	client, err := nativePrivacyCLIClient(options.timeout)
	if err != nil {
		return err
	}
	ctx, cancel := context.WithTimeout(cmd.Context(), options.timeout)
	defer cancel()
	result, err := nativePrivacyCLIQuery(ctx, client, cmd, options)
	if err != nil {
		return err
	}
	return json.NewEncoder(cmd.OutOrStdout()).Encode(result)
}

func nativePrivacyCLIClient(timeout time.Duration) (*apiclient.NativePrivacyClient, error) {
	config := libgoal.ClientConfig{AlgodDataDir: datadir.EnsureSingleDataDir()}
	client, err := libgoal.MakeClientFromConfig(config, libgoal.AlgodClient)
	if err != nil {
		return nil, err
	}
	return client.NativePrivacyQueries(&http.Client{Timeout: timeout})
}
