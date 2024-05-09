package apiserver

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/gommon/log"
	"github.com/suri312006/term2term/v2/internal/config"
	"github.com/suri312006/term2term/v2/internal/db"
)

type ServerConfig struct {
	Db db.Dbm
}

var e *echo.Echo


func Init(ec config.Env){
	e := echo.New()
	e.Logger.SetLevel(log.INFO)
	e.Logger.Fatal(e.Start(ec.Port))
}

