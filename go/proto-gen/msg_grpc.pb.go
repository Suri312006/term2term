// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             v5.27.0
// source: msg.proto

package v2

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	MsgService_Send_FullMethodName   = "/t2t.MsgService/Send"
	MsgService_Search_FullMethodName = "/t2t.MsgService/Search"
)

// MsgServiceClient is the client API for MsgService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type MsgServiceClient interface {
	Send(ctx context.Context, in *MsgSendReq, opts ...grpc.CallOption) (*Msg, error)
	Search(ctx context.Context, in *MsgSearchReq, opts ...grpc.CallOption) (*MsgList, error)
}

type msgServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewMsgServiceClient(cc grpc.ClientConnInterface) MsgServiceClient {
	return &msgServiceClient{cc}
}

func (c *msgServiceClient) Send(ctx context.Context, in *MsgSendReq, opts ...grpc.CallOption) (*Msg, error) {
	out := new(Msg)
	err := c.cc.Invoke(ctx, MsgService_Send_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *msgServiceClient) Search(ctx context.Context, in *MsgSearchReq, opts ...grpc.CallOption) (*MsgList, error) {
	out := new(MsgList)
	err := c.cc.Invoke(ctx, MsgService_Search_FullMethodName, in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MsgServiceServer is the server API for MsgService service.
// All implementations must embed UnimplementedMsgServiceServer
// for forward compatibility
type MsgServiceServer interface {
	Send(context.Context, *MsgSendReq) (*Msg, error)
	Search(context.Context, *MsgSearchReq) (*MsgList, error)
	mustEmbedUnimplementedMsgServiceServer()
}

// UnimplementedMsgServiceServer must be embedded to have forward compatible implementations.
type UnimplementedMsgServiceServer struct {
}

func (UnimplementedMsgServiceServer) Send(context.Context, *MsgSendReq) (*Msg, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Send not implemented")
}
func (UnimplementedMsgServiceServer) Search(context.Context, *MsgSearchReq) (*MsgList, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Search not implemented")
}
func (UnimplementedMsgServiceServer) mustEmbedUnimplementedMsgServiceServer() {}

// UnsafeMsgServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to MsgServiceServer will
// result in compilation errors.
type UnsafeMsgServiceServer interface {
	mustEmbedUnimplementedMsgServiceServer()
}

func RegisterMsgServiceServer(s grpc.ServiceRegistrar, srv MsgServiceServer) {
	s.RegisterService(&MsgService_ServiceDesc, srv)
}

func _MsgService_Send_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(MsgSendReq)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MsgServiceServer).Send(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: MsgService_Send_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MsgServiceServer).Send(ctx, req.(*MsgSendReq))
	}
	return interceptor(ctx, in, info, handler)
}

func _MsgService_Search_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(MsgSearchReq)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MsgServiceServer).Search(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: MsgService_Search_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MsgServiceServer).Search(ctx, req.(*MsgSearchReq))
	}
	return interceptor(ctx, in, info, handler)
}

// MsgService_ServiceDesc is the grpc.ServiceDesc for MsgService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var MsgService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "t2t.MsgService",
	HandlerType: (*MsgServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Send",
			Handler:    _MsgService_Send_Handler,
		},
		{
			MethodName: "Search",
			Handler:    _MsgService_Search_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "msg.proto",
}
