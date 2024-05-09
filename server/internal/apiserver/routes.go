package apiserver

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
)

func (a ApiServer) welcome(c echo.Context) error {
	time := a.db.PingTime()
	welcome := fmt.Sprintf(`
		Welcome to the Term2Term server!!!
		If you dont know what endpoints to hit, good luck!!

		METRICS:
		Database Time: %s
	`, time)
	return c.String(http.StatusOK, welcome)
}
