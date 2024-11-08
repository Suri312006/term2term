-- Add migration script here

ALTER TABLE Messages
RENAME COLUMN PubId TO MsgPubId;
