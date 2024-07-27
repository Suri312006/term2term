package server

import (
	"context"

	log "github.com/sirupsen/logrus"
	"github.com/suri312006/term2term/v2/internal/db"
	"github.com/suri312006/term2term/v2/internal/id"
	m "github.com/suri312006/term2term/v2/pkg/middleware"
	v2 "github.com/suri312006/term2term/v2/proto-gen"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

type UserServer struct {
	v2.UnimplementedUserServiceServer
}

func (s UserServer) Verify(ctx context.Context, req *v2.User) (*v2.VerifyUserRes, error) {

	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		log.Panic("database connection not provided")
	}

	userQuery := db.User{
		Name:  req.Name,
		PubId: req.Id,
	}

	foundUser := db.User{}

	if err := dbSesh.Query(&userQuery, &foundUser); err != nil {

	}

	if foundUser.PubId != "" {
		return &v2.VerifyUserRes{Verified: true}, nil
	}

	return &v2.VerifyUserRes{Verified: false}, nil

}

func (s UserServer) Search(ctx context.Context, req *v2.SearchUserReq) (*v2.UserList, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		log.Panic("database connection not provided")
	}

	// checking for all
	if v2.UserSearchTypes_name[0] == req.Kind.String() {
		users := []db.User{}
		dbSesh.QueryAll(&users)

		final := []*v2.User{}

		for _, user := range users {
			final = append(final, &v2.User{
				Id:   user.PubId,
				Name: user.Name,
			})
		}

		return &v2.UserList{
			Users: final,
		}, nil

	} else if true {
		return nil, status.Errorf(codes.Unimplemented, "method Search not implemented for this type of query")
	}

	return nil, status.Errorf(codes.Unimplemented, "method Search not implemented for this type of query")

}

func (s UserServer) Create(ctx context.Context, req *v2.NewUserReq) (*v2.User, error) {
	dbSesh := ctx.Value(m.DBSession).(*db.Dbm)
	if dbSesh == nil {
		log.Panic("database connection not provided") // do we want to leak this to user?
	}

	user_id := id.Must()

	user := db.User{
		PubId: user_id,
		Name:  req.Username,
	}

	err := dbSesh.Save(&user)
	if err != nil {
		panic(err)
	}

	log.Tracef("creating user: %v", user)

	return &v2.User{
		Id:   user.PubId,
		Name: user.Name,
	}, nil
}
