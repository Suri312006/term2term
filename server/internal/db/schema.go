package db

import (
	"gorm.io/gorm"
	"time"
)

type User struct {
	ID        uint           `json:"-" gorm:"primarykey"`
	CreatedAt time.Time      `json:"-"`
	UpdatedAt time.Time      `json:"-"`
	DeletedAt gorm.DeletedAt `gorm:"index"`

	// PrivateId string `json:"id" gorm:"uniqueIndex"`
	PubId string `json:"id" gorm:"uniqueIndex"`
	Name     string `json:"name"`
}

type Conversation struct {
	ID        uint           `json:"-" gorm:"primarykey"`
	CreatedAt time.Time      `json:"-"`
	UpdatedAt time.Time      `json:"-"`
	DeletedAt gorm.DeletedAt `gorm:"index"`

	PubId   string `json:"id" gorm:"uniqueIndex"`
	User1Id string `json:"user1_id"`
	User2Id string `json:"user2_id"`

	User1 User `gorm:"foreignKey:User1Id;references:PubId"`
	User2 User `gorm:"foreignKey:User2Id;references:PubId"`
}

type Message struct {
	ID        uint           `json:"-" gorm:"primarykey"`
	CreatedAt time.Time      `json:"-"`
	UpdatedAt time.Time      `json:"-"`
	DeletedAt gorm.DeletedAt `gorm:"index"`

	PubId string `json:"id" gorm:"uniqueIndex"`

	AuthorId string `json:"author"`
	Author   User   `gorm:"foreignKey:AuthorId;references:PubId"`

	RecipientId string `json:"recipient_id"`
	Recipient   User   `gorm:"foreignKey:RecipientId;references:PubId"`

	ConvoId string       `json:"convo_id"`
	Convo   Conversation `gorm:"foreignKey:ConversationId;references:PubId"`

	Body string `json:"body"`

	Read bool `json:"read"`
}



func (m Dbm) UpdateSchema() {
	m.db.AutoMigrate(&User{})
}

func (m Dbm) ResetSchema() {
	m.db.Migrator().DropTable(&User{}, &Conversation{}, &Message{})
	m.db.AutoMigrate(&User{}, &Conversation{}, &Message{})
}
