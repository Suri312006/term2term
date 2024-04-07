package db

type Message struct {
	Id string
	Author string
	Recipient string
	Body string
}

//TODO:  this should be a wrapper around an sqlite database
