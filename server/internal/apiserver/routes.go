package apiserver

import (
	"fmt"
	"net/http"
	"sync"

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

//TODO: need to get the user's name along with the query, this is gonna be 
//some relational stuff
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
