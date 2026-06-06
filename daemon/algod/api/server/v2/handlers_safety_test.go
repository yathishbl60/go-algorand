package v2

import (
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/labstack/echo/v4"
	"github.com/stretchr/testify/require"

	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/logging"
	"github.com/algorand/go-algorand/protocol"
	"github.com/algorand/go-algorand/test/partitiontest"
)

func TestDecodeSafetySimulatePayloadWrappedRequest(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	req := oneTxnGroupSafetyRequest()
	encoded := protocol.EncodeReflect(&PreEncodedSafetySimulateRequest{SimulateRequest: req, IncludeSimulation: true})
	decoded, includeSimulation, err := decodeSafetySimulatePayload(encoded)
	require.NoError(t, err)
	require.True(t, includeSimulation)
	require.Len(t, decoded.TxnGroups, 1)
	require.Len(t, decoded.TxnGroups[0].Txns, 1)
}

func TestDecodeSafetySimulatePayloadLegacyFallback(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	req := oneTxnGroupSafetyRequest()
	encoded := protocol.EncodeReflect(&req)
	decoded, includeSimulation, err := decodeSafetySimulatePayload(encoded)
	require.NoError(t, err)
	require.False(t, includeSimulation)
	require.Len(t, decoded.TxnGroups, 1)
	require.Len(t, decoded.TxnGroups[0].Txns, 1)
}

func TestDecodeSafetyEvaluatePayloadJSONFallback(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	req := PreEncodedSafetyEvaluateRequest{SimulateRequest: oneTxnGroupSafetyRequest(), Policy: SafetyPolicy{EnforcementMode: SafetyEnforcementHard}}
	encoded := protocol.EncodeJSON(&req)
	decoded, err := decodeSafetyEvaluatePayload(encoded)
	require.NoError(t, err)
	require.Equal(t, SafetyEnforcementHard, decoded.Policy.EnforcementMode)
	require.Len(t, decoded.SimulateRequest.TxnGroups, 1)
}

func TestDecodeSafetySimulatePayloadRejectsInvalidInput(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	_, _, err := decodeSafetySimulatePayload([]byte("not-valid-payload"))
	require.Error(t, err)
}

func TestGetCachedSimulateResponseTypeMismatch(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	h := Handlers{SafetyCache: newSafetyCache(8, time.Minute)}
	req := oneTxnGroupSafetyRequest()
	key := safetyCacheKey("simulate", PreEncodedSafetySimulateRequest{SimulateRequest: req, IncludeSimulation: false})
	h.SafetyCache.set(key, "wrong-type")

	_, returnedKey, ok := h.getCachedSimulateResponse(req, false)
	require.False(t, ok)
	require.Equal(t, key, returnedKey)
}

func TestGetCachedSimulateResponseIncludeSimulationKeyIsolation(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	h := Handlers{SafetyCache: newSafetyCache(8, time.Minute)}
	req := oneTxnGroupSafetyRequest()

	respWithSimulation := PreEncodedSafetySimulateResponse{OutcomeHash: "hash-with-sim", Simulation: &PreEncodedSimulateResponse{}}
	h.SafetyCache.set(safetyCacheKey("simulate", PreEncodedSafetySimulateRequest{SimulateRequest: req, IncludeSimulation: true}), respWithSimulation)

	_, _, ok := h.getCachedSimulateResponse(req, false)
	require.False(t, ok)

	resp, _, ok := h.getCachedSimulateResponse(req, true)
	require.True(t, ok)
	require.Equal(t, "hash-with-sim", resp.OutcomeHash)
	require.NotNil(t, resp.Simulation)
}

func TestGetCachedEvaluateResponseTypeMismatch(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	h := Handlers{SafetyCache: newSafetyCache(8, time.Minute)}
	req := PreEncodedSafetyEvaluateRequest{SimulateRequest: oneTxnGroupSafetyRequest()}
	key := safetyCacheKey("evaluate", req)
	h.SafetyCache.set(key, 42)

	_, returnedKey, ok := h.getCachedEvaluateResponse(req)
	require.False(t, ok)
	require.Equal(t, key, returnedKey)
}

func TestHandleSimulateSafetyErrorReturnsUnavailableResponse(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	h := Handlers{Log: logging.Base()}
	ctx, rec := newSafetyTestContext("/?format=json")
	err := h.handleSimulateSafetyError(ctx, errors.New("simulation backend unavailable"))
	require.NoError(t, err)
	require.Equal(t, http.StatusOK, rec.Code)

	var response PreEncodedSafetySimulateResponse
	require.NoError(t, protocol.DecodeJSON(rec.Body.Bytes(), &response))
	require.Equal(t, SafetyVerificationUnavailable, response.VerificationStatus)
	require.Contains(t, response.UnavailableReason, "unavailable")
}

func TestHandleEvaluateSafetyErrorRespectsBypassPolicy(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	allowBypass := false
	policy := SafetyPolicy{AllowBypassIfUnavailable: &allowBypass}
	h := Handlers{Log: logging.Base()}
	ctx, rec := newSafetyTestContext("/?format=json")
	err := h.handleEvaluateSafetyError(ctx, policy, errors.New("transient failure"))
	require.NoError(t, err)
	require.Equal(t, http.StatusOK, rec.Code)

	var response PreEncodedSafetyEvaluateResponse
	require.NoError(t, protocol.DecodeJSON(rec.Body.Bytes(), &response))
	require.Equal(t, SafetyDecisionUnverified, response.Decision)
	require.False(t, response.CanBypass)
	require.True(t, response.HardStop)
	require.False(t, response.Allowed)
}

func TestValidateSafetyTxnGroupsRejectsEmptyGroup(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	ctx, rec := newSafetyTestContext("/")
	err := validateSafetyTxnGroups(ctx, []PreEncodedSimulateRequestTransactionGroup{{Txns: nil}}, 1, logging.Base())
	require.NoError(t, err)
	require.Equal(t, http.StatusBadRequest, rec.Code)
}

func TestValidateSafetyTxnGroupsRejectsOversizedGroup(t *testing.T) {
	partitiontest.PartitionTest(t)
	t.Parallel()

	group := PreEncodedSimulateRequestTransactionGroup{Txns: []transactions.SignedTxn{{}, {}}}
	ctx, rec := newSafetyTestContext("/")
	err := validateSafetyTxnGroups(ctx, []PreEncodedSimulateRequestTransactionGroup{group}, 1, logging.Base())
	require.NoError(t, err)
	require.Equal(t, http.StatusBadRequest, rec.Code)
}

func oneTxnGroupSafetyRequest() PreEncodedSimulateRequest {
	return PreEncodedSimulateRequest{
		TxnGroups: []PreEncodedSimulateRequestTransactionGroup{{Txns: []transactions.SignedTxn{{}}}},
	}
}

func newSafetyTestContext(path string) (echo.Context, *httptest.ResponseRecorder) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodPost, path, nil)
	rec := httptest.NewRecorder()
	return e.NewContext(req, rec), rec
}
