package db

import (
	"time"

	"github.com/labstack/gommon/log"
	"github.com/suri312006/term2term/v2/internal/config"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

// db manager
type Dbm struct {
	db *gorm.DB
}

func Init(ec config.Env) Dbm {
	db, err := gorm.Open(postgres.Open(ec.DBString), &gorm.Config{})
	if err != nil {
		log.Fatal("failed to connect database", err)
	}
	sqldb, err := db.DB()

	//sets this on the underlying sql db
	sqldb.SetMaxIdleConns(10)
	sqldb.SetMaxOpenConns(100)
	sqldb.SetConnMaxLifetime(time.Hour)

	return Dbm{db}

}

// m for manager
func (m Dbm) PingTime() time.Time {
	var now time.Time
	_ = m.db.Raw("SELECT NOW()").Scan(&now)
	return now
}
