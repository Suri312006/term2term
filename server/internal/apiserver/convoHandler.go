package apiserver

import (
	"net/http"
	"sync"

	"github.com/labstack/echo/v4"
	"github.com/suri312006/term2term/v2/internal/db"
)

func (a ApiServer) initConvoRoutes(e *echo.Echo) {
	convoGroup := e.Group("/convo")
	convoGroup.GET("/list", a.listConversations)
}

// TODO: need to get the user's name along with the query, this is gonna be
// some relational stuff
func (a ApiServer) listConversations(c echo.Context) error {
	foundConvos1 := []db.Conversation{}
	foundConvos2 := []db.Conversation{}

	convoQuery1 := db.Conversation{
		User1Id: c.FormValue("userid"),
	}

	convoQuery2 := db.Conversation{
		User2Id: c.FormValue("userid"),
	}

	wg := sync.WaitGroup{}
	wg.Add(2)

	go func() {
		defer wg.Done()
		a.db.GroupQuery(&convoQuery1, &foundConvos1)
	}()

	go func() {
		defer wg.Done()
		go a.db.GroupQuery(&convoQuery2, &foundConvos2)
	}()

	foundConvos := append(foundConvos1, foundConvos2...)

	return c.JSON(http.StatusOK, foundConvos)
}
