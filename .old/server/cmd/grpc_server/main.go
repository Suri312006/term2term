package main

import (
	"context"
	"log"
	"net"

	"github.com/suri312006/term2term/v2/internal/config"
	"github.com/suri312006/term2term/v2/internal/db"
	"github.com/suri312006/term2term/v2/internal/id"
	v2 "github.com/suri312006/term2term/v2/proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	grpc_middleware "github.com/grpc-ecosystem/go-grpc-middleware"
)

type contextKey string

const (
	DBSession contextKey = "dbSession"
)

type server struct {
	v2.UnimplementedUserServiceServer
}

func (server) VerifyUser(ctx context.Context, req *v2.VerifyUserReq) (*v2.VerifyUserRes, error) {

	dbSesh := ctx.Value(DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}

	userQuery := db.User{
		Name:  req.Name,
		PubId: req.GetId(),
	}

	foundUser := db.User{}

	dbSesh.Query(&userQuery, &foundUser)

	return &v2.VerifyUserRes{
		Verified: foundUser.PubId != "",
	}, nil

}
func (server) SearchUser(context.Context, *v2.User) (*v2.UserList, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SearchUser not implemented")
}

func (server) NewUser(ctx context.Context, newUser *v2.NewUserReq) (*v2.User, error) {

	dbSesh := ctx.Value(DBSession).(*db.Dbm)
	if dbSesh == nil {
		return nil, status.Error(codes.Internal, "no database connection found")
	}

	user_id := id.Must()

	user := db.User{
		PubId: user_id,
		Name:  newUser.GetUsername(),
	}

	err := dbSesh.Save(&user)
	if err != nil {
		panic(err)
	}

	return &v2.User{
		Id:   user.PubId,
		Name: user.Name,
	}, nil

}

// func (v2.UnimplementedUserServiceServer) NewUser(ctx context.Context, newUser *v2.NewUserReq) (*v2.User, error) {
//
// 	dbSesh := ctx.Value(DBSession).(*db.Dbm)
// 	if dbSesh == nil {
// 		return nil, status.Error(codes.Internal, "no database connection found")
// 	}
//
// 	user_id := id.Must()
//
// 	user := db.User{
// 		PubId: user_id,
// 		Name:  newUser.GetUsername(),
// 	}
//
// 	err := dbSesh.Save(&user)
// 	if err != nil {
// 		panic(err)
// 	}
//
// 	return &pb.User{
// 		Id:   user.PubId,
// 		Name: user.Name,
// 	}, nil
// }
// func (v2.UnimplementedUserServiceServer) SearchUser(ctx context.Context, user *v2.User) (*v2.UserList, error) {
//
// 	dbSesh := ctx.Value(DBSession).(*db.Dbm)
// 	if dbSesh == nil {
// 		return nil, status.Error(codes.Internal, "no database connection found")
// 	}
// 	users := []db.User{}
//
// 	err := dbSesh.QueryAll(&users)
// 	if err != nil {
// 		return nil, status.Error(codes.Internal, "Error with database query")
// 	}
//
// 	final := []pb.User{}
//
// 	for _, v := range users {
// 		final = append(final, pb.User{
// 			Id:   v.PubId,
// 			Name: v.Name,
// 		})
//
// 	}
//
// 	return pb.UserList{
// 		Users: final,
// 	}, nil
// }
// func (v2.UnimplementedUserServiceServer) VerifyUser(context.Context, *v2.VerifyUserReq) (*v2.VerifyUserRes, error) {
// }
//
// // https://fale.io/blog/2022/03/21/inject-db-connections-in-golang-grpc-api
// func (v2.UnimplementedUserServiceServer) NewUser(ctx context.Context, newUser *v2.NewUserReq) (*v2.User, error) {
// }

func main() {

	cfg := config.Source()

	dbSession := db.Init(cfg)

	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("failed to listen on port 50051: %v", err)
	}

	s := grpc.NewServer(
		grpc.ChainStreamInterceptor(
			DBStreamServerInterceptor(&dbSession),
		),
		grpc.ChainUnaryInterceptor(
			DBUnaryServerInterceptor(&dbSession),
		),
	)

	v2.RegisterUserServiceServer(s, &server{})
	log.Printf("gRPC server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}

func DBUnaryServerInterceptor(session *db.Dbm) grpc.UnaryServerInterceptor {
	return func(ctx context.Context, req interface{}, info *grpc.UnaryServerInfo, handler grpc.UnaryHandler) (interface{}, error) {
		return handler(context.WithValue(ctx, DBSession, session), req)
	}
}

func DBStreamServerInterceptor(session *db.Dbm) grpc.StreamServerInterceptor {
	return func(srv interface{}, stream grpc.ServerStream, info *grpc.StreamServerInfo, handler grpc.StreamHandler) error {
		wrapped := grpc_middleware.WrapServerStream(stream)
		wrapped.WrappedContext = context.WithValue(stream.Context(), DBSession, session)
		return handler(srv, wrapped)
	}
}
