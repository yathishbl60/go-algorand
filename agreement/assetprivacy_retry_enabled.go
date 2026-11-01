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

package agreement

import (
	"context"
	"time"

	"github.com/algorand/go-algorand/data/transactions/verify"
)

// A local verifier failure cannot invalidate an already-filled proposal pipeline.
// Retry the same bounded request until it validates, rejects, or is cancelled.
func (c *poolCryptoVerifier) validateProposalWithLocalRetry(request cryptoProposalRequest) (proposal, error) {
	for {
		if err := c.nativePrivacyProposalCancellation(request.ctx); err != nil {
			return proposal{}, err
		}
		p, err := request.UnauthenticatedProposal.validate(request.ctx, request.Round, c.ledger, c.validator)
		if !verify.IsLocalNativePrivacyFailure(err) {
			return p, err
		}
		if err = c.waitNativePrivacyProposalRetry(request.ctx); err != nil {
			return proposal{}, err
		}
	}
}

func (c *poolCryptoVerifier) waitNativePrivacyProposalRetry(ctx context.Context) error {
	timer := time.NewTimer(10 * time.Millisecond)
	defer timer.Stop()
	select {
	case <-ctx.Done():
		return ctx.Err()
	case <-c.quit:
		return context.Canceled
	case <-timer.C:
		return nil
	}
}

func (c *poolCryptoVerifier) nativePrivacyProposalCancellation(ctx context.Context) error {
	select {
	case <-ctx.Done():
		return ctx.Err()
	case <-c.quit:
		return context.Canceled
	default:
		return nil
	}
}
