package apiserver

import (
	"net/http"

	"github.com/labstack/echo/v4"
	"github.com/suri312006/term2term/v2/internal/db"
	"github.com/suri312006/term2term/v2/internal/id"
)

func (a ApiServer) initMsgRoutes(e *echo.Echo) {
	msgGroup := e.Group("/msg")
	msgGroup.POST("/send", a.sendMessage)
}

func (a ApiServer) sendMessage(c echo.Context) error {

	msg := db.Message{
		PubId:       id.Must(),
		AuthorId:    c.FormValue("author_id"),
		RecipientId: c.FormValue("recipient_id"),
		ConvoId:     c.FormValue("convo_id"),
		Body:        c.FormValue("body"),
		Read:        false,
	}

	if err := a.db.Save(&msg); err != nil {
		return err
	}

	return c.String(http.StatusOK, "")
}
