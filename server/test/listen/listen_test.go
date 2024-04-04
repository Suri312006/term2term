package listen_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/suri312006/term2term/v2/internal/listen"
)

func TestConnectionHandler(t *testing.T) {

	t.Run("should return a 200 code if pinged", func(t *testing.T) {
		request, _ := http.NewRequest(http.MethodGet, "/listen", nil)
		response := httptest.NewRecorder()

		listen.ConnectionHandler(request, response)

		// unsure how this works

		want := http.StatusOK
		got := response.Code

		if got != want {
			t.Errorf("got %v want %v", got, want)
		}
	})

}
