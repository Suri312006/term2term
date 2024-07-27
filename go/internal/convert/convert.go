package convert

import (
	"github.com/suri312006/term2term/v2/internal/db"
	v2 "github.com/suri312006/term2term/v2/proto-gen"
)

func User2dbUser(u *v2.User) db.User {
	return db.User{
		PubId: u.Id,
		Name:  u.Name,
	}

}

func DbUser2User(u db.User) *v2.User {

	return &v2.User{
		Id:   u.PubId,
		Name: u.Name,
	}

}
