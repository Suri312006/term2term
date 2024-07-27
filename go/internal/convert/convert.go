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
func Convo2dbConvo(u *v2.Convo) db.Conversation {
	return db.Conversation{
		PubId:   u.Id,
		User1Id: u.Participants.Users[0].Id,
		User2Id: u.Participants.Users[1].Id,
		User1:   User2dbUser(u.Participants.Users[0]),
		User2:   User2dbUser(u.Participants.Users[1]),
	}
}

func DbConvo2Convo(u db.Conversation) *v2.Convo {

	return &v2.Convo{
		Id: u.PubId,
		Participants: &v2.Participants{
			Users: []*v2.User{
				DbUser2User(u.User1),
				DbUser2User(u.User2),
			},
		},
	}

}

func DbMsg2Msg(u db.Message) *v2.Msg {
	return &v2.Msg{
		Id:        u.PubId,
		Convo:     DbConvo2Convo(u.Convo),
		Author:    DbUser2User(u.Author),
		Recipient: DbUser2User(u.Recipient),
		Body:      u.Body,
		UnixTime:  uint64(u.CreatedAt.Unix()),
		IsRead:    u.Read,
	}

	// Id        string `protobuf:"bytes,1,opt,name=id,proto3" json:"id,omitempty"`
	// Convo     *Convo `protobuf:"bytes,2,opt,name=convo,proto3" json:"convo,omitempty"`
	// Author    *User  `protobuf:"bytes,3,opt,name=author,proto3" json:"author,omitempty"`
	// Recipient *User  `protobuf:"bytes,4,opt,name=recipient,proto3" json:"recipient,omitempty"`
	// Body      string `protobuf:"bytes,5,opt,name=body,proto3" json:"body,omitempty"`
	// UnixTime  uint64 `protobuf:"varint,6,opt,name=unix_time,json=unixTime,proto3" json:"unix_time,omitempty"`
	// IsRead    bool   `protobuf:"varint,7,opt,name=isRead,proto3" json:"isRead,omitempty"`
}
