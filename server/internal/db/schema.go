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
	PublicId  string `json:"id" gorm:"uniqueIndex"`
	Name      string `json:"name"`
}

type Conversation struct {
	ID        uint           `json:"-" gorm:"primarykey"`
	CreatedAt time.Time      `json:"-"`
	UpdatedAt time.Time      `json:"-"`
	DeletedAt gorm.DeletedAt `gorm:"index"`

	Id      string `json:"id" gorm:"uniqueIndex"`
	User1Id string `json:"user1"`
	User2Id string `json:"user2"`
}

func (m Dbm) UpdateSchema() {
	m.db.AutoMigrate(&User{})
}

func (m Dbm) ResetSchema() {
	m.db.Migrator().DropTable(&User{})
	m.db.AutoMigrate(&User{})

}
