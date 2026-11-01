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

package ledger

import (
	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/basics"
	"github.com/algorand/go-algorand/internal/assetprivacy/policy"
	"github.com/algorand/go-algorand/internal/assetprivacy/query"
	"github.com/algorand/go-algorand/internal/assetprivacy/suitecatalog"
)

// LookupNativeAssetPrivacy returns one owned, explicitly pinned public snapshot.
func (l *Ledger) LookupNativeAssetPrivacy(round basics.Round, asset basics.AssetIndex) (query.Snapshot, error) {
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	return l.nativePrivacySnapshot(round, asset)
}

func (l *Ledger) LookupLatestNativeAssetPrivacy(asset basics.AssetIndex) (query.Snapshot, error) {
	l.trackerMu.RLock()
	defer l.trackerMu.RUnlock()
	return l.nativePrivacySnapshot(l.blockQ.latest(), asset)
}

// The caller holds trackerMu throughout. DB retention errors propagate unchanged.
func (l *Ledger) nativePrivacySnapshot(round basics.Round, asset basics.AssetIndex) (query.Snapshot, error) {
	if asset == 0 {
		return query.Snapshot{}, query.ErrConfig
	}
	proto, err := l.nativePrivacyQueryProtocol(round)
	if err != nil {
		return query.Snapshot{}, err
	}
	return l.readNativePrivacySnapshot(proto, round, asset)
}

func (l *Ledger) readNativePrivacySnapshot(proto config.ConsensusParams, round basics.Round, asset basics.AssetIndex) (query.Snapshot, error) {
	config, err := l.nativePrivacyQueryConfig(round, asset)
	if err != nil {
		return query.Snapshot{}, err
	}
	snapshot, err := query.Read(nativePrivacyQueryKV{&l.accts, round}, config)
	if err != nil {
		return query.Snapshot{}, err
	}
	if err = validateNativePrivacyQuerySuite(proto, snapshot); err != nil {
		return query.Snapshot{}, err
	}
	return snapshot, nil
}

func (l *Ledger) nativePrivacyQueryProtocol(round basics.Round) (config.ConsensusParams, error) {
	header, err := l.BlockHdr(round)
	if err != nil {
		return config.ConsensusParams{}, err
	}
	proto, found := config.Consensus[header.CurrentProtocol]
	if !found || !proto.EnableNativeAssetPrivacy {
		return config.ConsensusParams{}, policy.ErrDisabled
	}
	return proto, nil
}

func validateNativePrivacyQuerySuite(proto config.ConsensusParams, snapshot query.Snapshot) error {
	if snapshot.Policy != nil {
		_, err := suitecatalog.Lookup(proto, snapshot.Policy.Suite)
		return err
	}
	if snapshot.Closure != nil {
		_, err := suitecatalog.Lookup(proto, snapshot.Closure.Suite)
		return err
	}
	return nil
}
