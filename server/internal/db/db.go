package db

import "os"

// import "path/filepath"

//TODO: create a local database for now that can store user and messages

type Db struct {
	Filepath string
	file     *os.File
}


func New(filePath string) Db {
	file, err := os.Create(filePath)
	defer file.Close()

	if err != nil {
		panic("was not able to create database file")
	}
	return Db{Filepath: filePath,
		file: file}

}
func (d Db) DeleteDb() {
	err := os.Remove(d.Filepath)
	if err != nil {
		panic("Unable to delete databse")
	}
}
