package listen_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/suri312006/term2term/v2/internal/listen"
)

func TestConnectionHandler(t *testing.T) {

	t.Run("should return a 200 code if pinged", func(t *testing.T) {
		req, _ := http.NewRequest(http.MethodGet, "/listen", nil)
		res := httptest.NewRecorder()

		server := listen.Server{}

		server.ServeHttp(res, req)

		got := res.Code
		want := http.StatusOK

		if got != want {
			t.Errorf("got %v want %v", got, want)
		}
	})

}
