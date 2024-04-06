package listen_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	api "github.com/suri312006/term2term/v2/internal/api"
)

func TestServeHTTP(t *testing.T) {

	t.Run("should return a 200 code if pinged", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodGet, "/", nil)
		res := httptest.NewRecorder()

		server := api.Server{}

		server.ServeHTTP(res, req)

		assertCode(t, res, http.StatusOK)
	})

	t.Run("Should return status accepted on POST request ", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodPost, "/", nil)
		res := httptest.NewRecorder()

		server := api.Server{}

		server.ServeHTTP(res, req)

		assertCode(t, res, http.StatusAccepted)
	})

}
func assertCode(t testing.TB, got *httptest.ResponseRecorder, wantedCode int) {
	t.Helper()
	if got.Code != wantedCode {
		t.Errorf("incorrect code, got %v want %v", got.Code, wantedCode)
	}

}
