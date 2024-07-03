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
	Username string `json:"username"`
}
