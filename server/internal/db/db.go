package db

import (
	"encoding/json"
	"fmt"
	"os"
)

// import "path/filepath"

type Entry struct {
	Id string
	Author string
	Message string
	Recipient string
}

type Db struct {
	Filepath string
}

func New(filePath string) Db {
	_, err := os.Create(filePath)

	if err != nil {
		panic("was not able to create database file")
	}

	return Db{Filepath: filePath}

}

func (d Db) InsertDB(entry Entry) {
	
	file := d.openFile()
	defer file.Close()

	enc := json.NewEncoder(file)

	err := enc.Encode(entry)

	if err != nil {
		panic(err)
	}
}

func (d Db) GetEntryById(id string)(*Entry, error){

	file := d.openFile()
	defer file.Close()

	dec := json.NewDecoder(file)	

	var entries map[string]interface{}

	dec.Decode(&entries)


	for i, v := range entries{
		fmt.Printf("%v %v", i, v)
	}
	

	return &Entry{}, nil

}


func (d Db) DeleteDb() {
	
	err := os.Remove(d.Filepath)

	if err != nil {
		panic("Unable to delete databse")
	}
}

func (d Db) openFile() *os.File {
	//NOTE: append file permission so we can get read / write access
	file, err := os.OpenFile(d.Filepath, os.O_RDWR, os.ModeAppend)

	if err != nil {
		panic("unable to open db")
	}

	return file
}
