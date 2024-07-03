package apiserver

import (
	"fmt"
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/suri312006/term2term/v2/internal/db"
	"github.com/suri312006/term2term/v2/internal/id"
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

func (a ApiServer) registerUser(c echo.Context) error {

	user_id := id.Must()

	user := db.User{
		PublicId: user_id,
		Username: c.FormValue("username"),
	}

	a.db.Save(&user)

	return c.JSON(http.StatusOK, user)
}
