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

func (m Dbm) AutoCreateStruct(data interface{}) error {
	err := m.db.AutoMigrate(data)
	return err
}

func (m Dbm) Save(data interface{}) error {
	res := m.db.Create(data)
	return res.Error
}

func (m Dbm) Query(data, store interface{}) error {
	res := m.db.Where(data).First(store)
	return res.Error
}

func (m Dbm) GroupQuery(model, store interface{}) error {
	res := m.db.Where(model).Find(store)
	return res.Error
}

func (m Dbm) QueryAll(store interface{}) error {
	res := m.db.Find(store)
	return res.Error
}

func (m Dbm) Update(model, data interface{}) error {
	res := m.db.Model(model).Updates(data)
	return res.Error
}

func (m Dbm) Delete(data interface{}) error {
	res := m.db.Delete(data)
	return res.Error
}

func (m Dbm) CreateAssociation(model interface{}, key string, value interface{}) error {
	err := m.db.Model(model).Association(key).Append(value)
	return err
}

func (m Dbm) ReadAssociation(model interface{}, key string, store interface{}) error {
	err := m.db.Model(model).Association(key).Find(store)
	return err
}

func (m Dbm) UpdateAssociation(model interface{}, key string, value interface{}) error {
	err := m.db.Model(model).Association(key).Replace(value)
	return err
}

func (m Dbm) DeleteAssociation(model interface{}, key string, value interface{}) error {
	err := m.db.Model(model).Association(key).Delete(value)
	return err
}

func (m Dbm) ClearAssociations(model interface{}, key string) error {
	err := m.db.Model(model).Association(key).Clear()
	return err
}
