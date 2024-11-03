-- Add migration script here
-- create the password column
ALTER TABLE Users
ADD HashedPass TEXT;

-- set a default for all existing ones
UPDATE Users SET HashedPass = 'VOID_PASSWORD';

-- then make that column not null
ALTER TABLE Users
ALTER COLUMN HashedPass SET NOT NULL;



