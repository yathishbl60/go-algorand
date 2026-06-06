package test

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/require"

	"github.com/algorand/go-algorand/config"
	v2 "github.com/algorand/go-algorand/daemon/algod/api/server/v2"
	"github.com/algorand/go-algorand/daemon/algod/api/server/v2/generated/model"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/ledger/simulation"
	"github.com/algorand/go-algorand/logging"
	"github.com/algorand/go-algorand/protocol"
	"github.com/algorand/go-algorand/test/partitiontest"
)

func TestSimulateSafetyEndpointWithWrappedRequest(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	handler, stxn, release := setupSafetyHandler(t)
	defer release()

	request := v2.PreEncodedSafetySimulateRequest{
		SimulateRequest: v2.PreEncodedSimulateRequest{
			TxnGroups: []v2.PreEncodedSimulateRequestTransactionGroup{{Txns: []transactions.SignedTxn{stxn}}},
		},
		IncludeSimulation: true,
	}

	body := protocol.EncodeReflect(&request)
	req := httptest.NewRequest(http.MethodPost, "/?format=json", bytes.NewReader(body))
	rec := httptest.NewRecorder()
	c := echo.New().NewContext(req, rec)

	err := handler.SimulateSafety(c)
	require.NoError(t, err)
	require.Equal(t, http.StatusOK, rec.Code)

	var response v2.PreEncodedSafetySimulateResponse
	require.NoError(t, protocol.DecodeJSON(rec.Body.Bytes(), &response))
	require.Equal(t, v2.SafetyVerificationVerified, response.VerificationStatus)
	require.NotEmpty(t, response.OutcomeHash)
	require.NotNil(t, response.Simulation)
}

func TestSimulateSafetyEndpointAcceptsLegacyRequest(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	handler, stxn, release := setupSafetyHandler(t)
	defer release()

	legacy := v2.PreEncodedSimulateRequest{
		TxnGroups: []v2.PreEncodedSimulateRequestTransactionGroup{{Txns: []transactions.SignedTxn{stxn}}},
	}

	body := protocol.EncodeReflect(&legacy)
	req := httptest.NewRequest(http.MethodPost, "/?format=json", bytes.NewReader(body))
	rec := httptest.NewRecorder()
	c := echo.New().NewContext(req, rec)

	err := handler.SimulateSafety(c)
	require.NoError(t, err)
	require.Equal(t, http.StatusOK, rec.Code)

	var response v2.PreEncodedSafetySimulateResponse
	require.NoError(t, protocol.DecodeJSON(rec.Body.Bytes(), &response))
	require.Equal(t, v2.SafetyVerificationVerified, response.VerificationStatus)
	require.NotEmpty(t, response.OutcomeHash)
	require.Nil(t, response.Simulation)
}

func TestEvaluateSafetyEndpointJSONRequest(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	handler, stxn, release := setupSafetyHandler(t)
	defer release()

	evaluate := v2.PreEncodedSafetyEvaluateRequest{
		SimulateRequest: v2.PreEncodedSimulateRequest{
			TxnGroups: []v2.PreEncodedSimulateRequestTransactionGroup{{Txns: []transactions.SignedTxn{stxn}}},
		},
		Policy: v2.SafetyPolicy{EnforcementMode: v2.SafetyEnforcementSoft},
	}

	body := protocol.EncodeJSON(&evaluate)
	req := httptest.NewRequest(http.MethodPost, "/?format=json", bytes.NewReader(body))
	rec := httptest.NewRecorder()
	c := echo.New().NewContext(req, rec)

	err := handler.EvaluateSafety(c)
	require.NoError(t, err)
	require.Equal(t, http.StatusOK, rec.Code)

	var response v2.PreEncodedSafetyEvaluateResponse
	require.NoError(t, protocol.DecodeJSON(rec.Body.Bytes(), &response))
	require.Equal(t, v2.SafetyVerificationVerified, response.VerificationStatus)
	require.True(t, response.Allowed)
	require.Equal(t, v2.SafetyDecisionAllow, response.Decision)
	require.NotEmpty(t, response.OutcomeHash)
}

func TestSimulateSafetyEndpointRejectsTraceWhenDeveloperAPIDisabled(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	handler, stxn, release := setupSafetyHandlerWithDeveloperAPI(t, false)
	defer release()

	request := v2.PreEncodedSafetySimulateRequest{
		SimulateRequest: v2.PreEncodedSimulateRequest{
			TxnGroups:       []v2.PreEncodedSimulateRequestTransactionGroup{{Txns: []transactions.SignedTxn{stxn}}},
			ExecTraceConfig: simulation.ExecTraceConfig{Enable: true},
		},
	}

	body := protocol.EncodeReflect(&request)
	req := httptest.NewRequest(http.MethodPost, "/?format=json", bytes.NewReader(body))
	rec := httptest.NewRecorder()
	c := echo.New().NewContext(req, rec)

	err := handler.SimulateSafety(c)
	require.NoError(t, err)
	require.Equal(t, http.StatusBadRequest, rec.Code)

	var response model.ErrorResponse
	require.NoError(t, protocol.DecodeJSON(rec.Body.Bytes(), &response))
	require.Contains(t, response.Message, "EnableDeveloperAPI")
}

func setupSafetyHandler(t *testing.T) (v2.Handlers, transactions.SignedTxn, func()) {
	t.Helper()
	return setupSafetyHandlerWithDeveloperAPI(t, true)
}

func setupSafetyHandlerWithDeveloperAPI(t *testing.T, enabled bool) (v2.Handlers, transactions.SignedTxn, func()) {
	t.Helper()

	mockLedger, _, _, stxns, release := testingenv(t, 2, 1, true)
	cfg := config.GetDefaultLocal()
	cfg.EnableDeveloperAPI = enabled
	mockNode := makeMockNodeWithConfig(mockLedger, t.Name(), nil, cannedStatusReportGolden, false, cfg)

	handler := v2.Handlers{
		Node:        mockNode,
		Log:         logging.Base(),
		Shutdown:    make(chan struct{}),
		SafetyCache: v2.NewDefaultSafetyCache(),
	}
	return handler, stxns[0], release
}
