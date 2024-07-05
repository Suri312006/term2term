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

	PublicId string `json:"id" gorm:"uniqueIndex"`
	Name string `json:"username"`
}

func (m Dbm) UpdateSchema() {
	m.db.AutoMigrate(&User{})
}

func (m Dbm) ResetSchema() {
	m.db.Migrator().DropTable(&User{})
	m.db.AutoMigrate(&User{})

}
