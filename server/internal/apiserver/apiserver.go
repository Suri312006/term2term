package apiserver

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/labstack/gommon/log"
	"github.com/suri312006/term2term/v2/internal/config"
	"github.com/suri312006/term2term/v2/internal/db"
)

type ApiServer struct {
	e    *echo.Echo
	db   db.Dbm
	port string
}

func Init(ec config.Env, db db.Dbm) ApiServer {
	e := echo.New()
	e.Logger.SetLevel(log.INFO)
	e.Use(middleware.Logger())
	return ApiServer{
		e,
		db,
		ec.Port,
	}
}

func (a ApiServer) Start() {
	a.initRoutes()

	a.db.PingTime()
	a.e.Logger.Fatal(a.e.Start(a.port))
}

func (a ApiServer) initRoutes() {

	a.e.GET("/", a.welcome)

	a.e.POST("/user/register", a.registerUser)

	a.e.POST("/user/verify", a.verifyUser)

}
