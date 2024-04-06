package db_test

import (
	Db "github.com/suri312006/term2term/v2/internal/db"
	"os"
	"testing"
)

const testDbFilePath = "/home/suri/coding/term2term/server/test/db/test_db.json"

func TestDb(t *testing.T) {
	t.Run("Creates a new empty db.json on initialization", func(t *testing.T) {

		// delete file if it exists just in case
		err := os.Remove(testDbFilePath)
		if err != nil {
			println(err)
		}

		Db.New(testDbFilePath)

		fileInfo, err := os.Stat(testDbFilePath)

		if err != nil {
			t.Fatal("Was not able to stat directory")
		}

		if fileInfo.Size() != 0 {
			t.Errorf("File was not empyty")
		}

	})
}
