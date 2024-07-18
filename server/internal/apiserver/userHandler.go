package apiserver

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/suri312006/term2term/v2/internal/db"
	"github.com/suri312006/term2term/v2/internal/id"
)

func (a ApiServer) initUserRoutes(e *echo.Echo) {
	userGroup := e.Group("/user")
	userGroup.POST("/register", a.registerUser)
	userGroup.GET("/verify", a.verifyUser)
}

func (a ApiServer) registerUser(c echo.Context) error {

	user_id := id.Must()

	user := db.User{
		PublicId: user_id,
		Name:     c.FormValue("name"),
	}

	err := a.db.Save(&user)
	if err != nil {
		panic(err)
	}

	return c.JSON(http.StatusOK, user)
}
func (a ApiServer) verifyUser(c echo.Context) error {

	userQuery := db.User{
		Name:     c.FormValue("name"),
		PublicId: c.FormValue("id"),
	}

	foundUser := db.User{}

	a.db.Query(&userQuery, &foundUser)

	if foundUser.PublicId != "" {
		return c.JSON(http.StatusOK, map[string]bool{
			"verified": true,
		})
	}

	return c.JSON(http.StatusOK, map[string]bool{
		"verified": false,
	})
}
