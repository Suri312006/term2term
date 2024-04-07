package db_test

import (
	"os"
	"testing"

	"github.com/suri312006/term2term/v2/internal/db"
	Db "github.com/suri312006/term2term/v2/internal/db"
)

const testDbFilePath = "/home/suri/coding/term2term/server/test/db/test_db.json"

func TestNew(t *testing.T) {
	t.Run("Creates a new empty db.json on initialization", func(t *testing.T) {

		// delete file if it exists just in case
		err := os.Remove(testDbFilePath)
		if err != nil {
			println(err)
		}

		db := Db.New(testDbFilePath)
		defer db.DeleteDb()

		fileInfo, err := os.Stat(testDbFilePath)

		if err != nil {
			t.Fatal("Was not able to stat directory")
		}

		if fileInfo.Size() != 0 {
			t.Errorf("File was not empyty")
		}

	})
}

func TestDeleteDb(t *testing.T) {
	t.Run("checks if database is deleted after method call", func(t *testing.T) {

		db := db.New(testDbFilePath)

		db.DeleteDb()

		_, err := os.Stat(testDbFilePath)

		if err == nil {
			t.Errorf("%v", err)
		}

	})
}

func TestInsertDB(t *testing.T) {
	t.Run("Asserts that you can enter data into db", func(t *testing.T) {

	})
}
