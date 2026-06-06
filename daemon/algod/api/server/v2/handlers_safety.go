package v2

import (
	"bytes"
	"errors"
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"

	"github.com/algorand/go-algorand/config"
	"github.com/algorand/go-algorand/data/transactions"
	"github.com/algorand/go-algorand/ledger/simulation"
	"github.com/algorand/go-algorand/logging"
	"github.com/algorand/go-algorand/protocol"
)

// SimulateSafety predicts outcomes and returns deterministic safety metadata.
func (v2 *Handlers) SimulateSafety(ctx echo.Context) error {
	simulateRequest, includeSimulation, proto, err := v2.decodeSafetySimulateRequest(ctx)
	if err != nil {
		return err
	}
	if err = validateSafetyTxnGroups(ctx, simulateRequest.TxnGroups, proto.MaxTxGroupSize, v2.Log); err != nil {
		return err
	}

	cacheKey := ""
	if safetyResponse, key, ok := v2.getCachedSimulateResponse(simulateRequest, includeSimulation); ok {
		return v2.writeSafetyResponse(ctx, safetyResponse)
	} else {
		cacheKey = key
	}

	simResult, err := v2.Node.Simulate(convertSimulationRequest(simulateRequest))
	if err != nil {
		return v2.handleSimulateSafetyError(ctx, err)
	}

	response := convertSimulationResult(simResult)
	safetyResponse := buildVerifiedSimulateSafetyResponse(response)
	if includeSimulation {
		safetyResponse.Simulation = &response
	}
	if v2.SafetyCache != nil && cacheKey != "" {
		v2.SafetyCache.set(cacheKey, safetyResponse)
	}
	return v2.writeSafetyResponse(ctx, safetyResponse)
}

func (v2 *Handlers) writeSafetyResponse(ctx echo.Context, response PreEncodedSafetySimulateResponse) error {

	format := ctx.QueryParam("format")
	var formatPtr *string
	if format != "" {
		formatPtr = &format
	}
	handle, contentType, err := getCodecHandle(formatPtr)
	if err != nil {
		return badRequest(ctx, err, errFailedParsingFormatOption, v2.Log)
	}
	responseData, err := encode(handle, &response)
	if err != nil {
		return internalError(ctx, err, errFailedToEncodeResponse, v2.Log)
	}
	return ctx.Blob(http.StatusOK, contentType, responseData)
}

// EvaluateSafety simulates and checks a policy against predicted effects.
func (v2 *Handlers) EvaluateSafety(ctx echo.Context) error {
	request, proto, err := v2.decodeSafetyEvaluateRequest(ctx)
	if err != nil {
		return err
	}
	if err = validateSafetyTxnGroups(ctx, request.SimulateRequest.TxnGroups, proto.MaxTxGroupSize, v2.Log); err != nil {
		return err
	}

	cacheKey := ""
	if evaluateResponse, key, ok := v2.getCachedEvaluateResponse(request); ok {
		return v2.writeEvaluateSafetyResponse(ctx, evaluateResponse)
	} else {
		cacheKey = key
	}

	simResult, err := v2.Node.Simulate(convertSimulationRequest(request.SimulateRequest))
	if err != nil {
		return v2.handleEvaluateSafetyError(ctx, request.Policy, err)
	}

	evaluateResponse := buildVerifiedEvaluateSafetyResponse(convertSimulationResult(simResult), request.Policy)
	if v2.SafetyCache != nil && cacheKey != "" {
		v2.SafetyCache.set(cacheKey, evaluateResponse)
	}
	return v2.writeEvaluateSafetyResponse(ctx, evaluateResponse)
}

func (v2 *Handlers) writeEvaluateSafetyResponse(ctx echo.Context, response PreEncodedSafetyEvaluateResponse) error {

	format := ctx.QueryParam("format")
	var formatPtr *string
	if format != "" {
		formatPtr = &format
	}
	handle, contentType, err := getCodecHandle(formatPtr)
	if err != nil {
		return badRequest(ctx, err, errFailedParsingFormatOption, v2.Log)
	}
	responseData, err := encode(handle, &response)
	if err != nil {
		return internalError(ctx, err, errFailedToEncodeResponse, v2.Log)
	}
	return ctx.Blob(http.StatusOK, contentType, responseData)
}

func (v2 *Handlers) decodeSafetySimulateRequest(ctx echo.Context) (PreEncodedSimulateRequest, bool, config.ConsensusParams, error) {
	stat, err := v2.Node.Status()
	if err != nil {
		return PreEncodedSimulateRequest{}, false, config.ConsensusParams{}, internalError(ctx, err, errFailedRetrievingNodeStatus, v2.Log)
	}
	proto := config.Consensus[stat.LastVersion]

	requestData, err := readSafetyRequestData(ctx)
	if err != nil {
		return PreEncodedSimulateRequest{}, false, config.ConsensusParams{}, badRequest(ctx, err, err.Error(), v2.Log)
	}

	simulateRequest, includeSimulation, err := decodeSafetySimulatePayload(requestData)
	if err != nil {
		return PreEncodedSimulateRequest{}, false, config.ConsensusParams{}, badRequest(ctx, err, err.Error(), v2.Log)
	}
	return simulateRequest, includeSimulation, proto, nil
}

func (v2 *Handlers) decodeSafetyEvaluateRequest(ctx echo.Context) (PreEncodedSafetyEvaluateRequest, config.ConsensusParams, error) {
	stat, err := v2.Node.Status()
	if err != nil {
		return PreEncodedSafetyEvaluateRequest{}, config.ConsensusParams{}, internalError(ctx, err, errFailedRetrievingNodeStatus, v2.Log)
	}
	proto := config.Consensus[stat.LastVersion]

	requestData, err := readSafetyRequestData(ctx)
	if err != nil {
		return PreEncodedSafetyEvaluateRequest{}, config.ConsensusParams{}, badRequest(ctx, err, err.Error(), v2.Log)
	}

	request, err := decodeSafetyEvaluatePayload(requestData)
	if err != nil {
		return PreEncodedSafetyEvaluateRequest{}, config.ConsensusParams{}, badRequest(ctx, err, err.Error(), v2.Log)
	}
	return request, proto, nil
}

func (v2 *Handlers) getCachedSimulateResponse(request PreEncodedSimulateRequest, includeSimulation bool) (PreEncodedSafetySimulateResponse, string, bool) {
	if v2.SafetyCache == nil {
		return PreEncodedSafetySimulateResponse{}, "", false
	}
	key := safetyCacheKey("simulate", PreEncodedSafetySimulateRequest{SimulateRequest: request, IncludeSimulation: includeSimulation})
	cached, ok := v2.SafetyCache.get(key)
	if !ok {
		return PreEncodedSafetySimulateResponse{}, key, false
	}
	response, typeOK := cached.(PreEncodedSafetySimulateResponse)
	if !typeOK {
		return PreEncodedSafetySimulateResponse{}, key, false
	}
	return response, key, true
}

func (v2 *Handlers) getCachedEvaluateResponse(request PreEncodedSafetyEvaluateRequest) (PreEncodedSafetyEvaluateResponse, string, bool) {
	if v2.SafetyCache == nil {
		return PreEncodedSafetyEvaluateResponse{}, "", false
	}
	key := safetyCacheKey("evaluate", request)
	cached, ok := v2.SafetyCache.get(key)
	if !ok {
		return PreEncodedSafetyEvaluateResponse{}, key, false
	}
	response, typeOK := cached.(PreEncodedSafetyEvaluateResponse)
	if !typeOK {
		return PreEncodedSafetyEvaluateResponse{}, key, false
	}
	return response, key, true
}

func (v2 *Handlers) handleSimulateSafetyError(ctx echo.Context, err error) error {
	var invalidTxErr simulation.InvalidRequestError
	if errors.As(err, &invalidTxErr) {
		return badRequest(ctx, invalidTxErr, invalidTxErr.Error(), v2.Log)
	}
	return v2.writeSafetyResponse(ctx, unverifiedSimulateResponse(err.Error()))
}

func (v2 *Handlers) handleEvaluateSafetyError(ctx echo.Context, policy SafetyPolicy, err error) error {
	var invalidTxErr simulation.InvalidRequestError
	if errors.As(err, &invalidTxErr) {
		return badRequest(ctx, invalidTxErr, invalidTxErr.Error(), v2.Log)
	}
	return v2.writeEvaluateSafetyResponse(ctx, unverifiedEvaluateResponse(policy, err.Error()))
}

func buildVerifiedSimulateSafetyResponse(response PreEncodedSimulateResponse) PreEncodedSafetySimulateResponse {
	riskSignals := transactions.CollectGroupRiskSignals(buildAttestedTxGroup(response))
	return PreEncodedSafetySimulateResponse{
		OutcomeSummary:     buildSafetyOutcomeSummary(response),
		OutcomeHash:        computeSafetyOutcomeHash(response),
		RiskSignals:        riskSignals,
		RiskLevel:          deriveRiskLevel(riskSignals),
		VerificationStatus: SafetyVerificationVerified,
	}
}

func buildVerifiedEvaluateSafetyResponse(response PreEncodedSimulateResponse, policy SafetyPolicy) PreEncodedSafetyEvaluateResponse {
	summary := buildSafetyOutcomeSummary(response)
	violations := evaluateSafetyPolicy(summary, policy)
	decision, hardStop, canBypass, enforcement, allowed := summarizePolicyDecision(policy, violations)
	return PreEncodedSafetyEvaluateResponse{
		Allowed:            allowed,
		Violations:         violations,
		Decision:           decision,
		HardStop:           hardStop,
		CanBypass:          canBypass,
		EnforcementMode:    enforcement,
		VerificationStatus: SafetyVerificationVerified,
		OutcomeSummary:     summary,
		OutcomeHash:        computeSafetyOutcomeHash(response),
	}
}

func validateSafetyTxnGroups(ctx echo.Context, groups []PreEncodedSimulateRequestTransactionGroup, maxTxGroupSize int, log logging.Logger) error {
	for _, txgroup := range groups {
		if len(txgroup.Txns) == 0 {
			err := errors.New("empty txgroup")
			return badRequest(ctx, err, err.Error(), log)
		}
		if len(txgroup.Txns) > maxTxGroupSize {
			err := fmt.Errorf("transaction group size %d exceeds protocol max %d", len(txgroup.Txns), maxTxGroupSize)
			return badRequest(ctx, err, err.Error(), log)
		}
	}
	return nil
}

func readSafetyRequestData(ctx echo.Context) ([]byte, error) {
	requestBuffer := new(bytes.Buffer)
	requestBodyReader := http.MaxBytesReader(nil, ctx.Request().Body, MaxTealDryrunBytes)
	if _, err := requestBuffer.ReadFrom(requestBodyReader); err != nil {
		return nil, err
	}
	return requestBuffer.Bytes(), nil
}

func decodeSafetySimulatePayload(requestData []byte) (PreEncodedSimulateRequest, bool, error) {
	var request PreEncodedSafetySimulateRequest
	wrappedErr := decodeSafetyPayload(requestData, &request)
	if wrappedErr == nil && len(request.SimulateRequest.TxnGroups) > 0 {
		return request.SimulateRequest, request.IncludeSimulation, nil
	}

	var fallback PreEncodedSimulateRequest
	fallbackErr := decodeSafetyPayload(requestData, &fallback)
	if fallbackErr != nil {
		if wrappedErr != nil {
			return PreEncodedSimulateRequest{}, false, wrappedErr
		}
		return PreEncodedSimulateRequest{}, false, errors.New("missing transaction group")
	}
	if len(fallback.TxnGroups) == 0 {
		return PreEncodedSimulateRequest{}, false, errors.New("missing transaction group")
	}
	return fallback, false, nil
}

func decodeSafetyEvaluatePayload(requestData []byte) (PreEncodedSafetyEvaluateRequest, error) {
	var request PreEncodedSafetyEvaluateRequest
	if err := decodeSafetyPayload(requestData, &request); err != nil {
		return PreEncodedSafetyEvaluateRequest{}, err
	}
	return request, nil
}

func decodeSafetyPayload(requestData []byte, out any) error {
	err := decode(protocol.CodecHandle, requestData, out)
	if err == nil {
		return nil
	}
	if jsonErr := decode(protocol.JSONStrictHandle, requestData, out); jsonErr == nil {
		return nil
	}
	return err
}
