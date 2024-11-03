-- Add migration script here

-- delete any duplicate email entries that already exist
DELETE FROM Users
WHERE Email IN (
  SELECT Email
  FROM Users
  GROUP BY Email
  HAVING COUNT(*) > 1
)
AND UserId NOT IN (
  SELECT MIN(UserId)
  FROM Users
  GROUP BY Email
  HAVING COUNT(*) > 1
);

ALTER TABLE Users
ADD CONSTRAINT unique_email UNIQUE (Email);
