-- Add migration script here
ALTER TABLE Users
ADD Email varchar(255);

UPDATE Users SET Email = 'VOID_EMAIL';

ALTER TABLE Users
ALTER COLUMN Email SET NOT NULL;
