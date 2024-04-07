package listen

import (
	"fmt"
	"net/http"
)

type Server struct {
}

func (s Server) ServeHTTP(w http.ResponseWriter, r *http.Request) {

	if r.Method == http.MethodPost {
		w.WriteHeader(http.StatusAccepted)
		return
	}

	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "hi albert pookie!!!")

}
