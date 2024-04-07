package listen

import (
	// "encoding/json"
	"fmt"
	// "log"
	"net/http"
	// DB "github.com/suri312006/term2term/v2/internal/db"
)

type Server struct {
}

func (s Server) ServeHTTP(w http.ResponseWriter, r *http.Request) {

	// just matches patterns
	mux := http.NewServeMux()

	mux.HandleFunc("/", handleRoot)

	mux.ServeHTTP(w, r)
}

func handleRoot(w http.ResponseWriter, r *http.Request) {

	switch r.Method {
	case http.MethodPost:
		post(w, r)
	case http.MethodGet:
		get(w, r)
	}

}

func get(w http.ResponseWriter, r *http.Request) {

	fmt.Fprintf(w, "hi albert pookie!!!")
	w.WriteHeader(http.StatusOK)

}

func post(w http.ResponseWriter, r *http.Request) {

	r.ParseForm()

	for k, v := range r.Form {
		fmt.Printf("%s = %s \n", k, v)
	}

	w.WriteHeader(http.StatusAccepted)

}
