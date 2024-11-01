-- initial sql script to create this shit
CREATE TABLE Users (
    UserId SERIAL PRIMARY KEY,
    UserPubId varchar(255) NOT NULL,
    CreatedAt TIMESTAMPTZ DEFAULT now() NOT NULL,
    UpdatedAt TIMESTAMPTZ DEFAULT now() NOT NULL,

    Name varchar(255) NOT NULL,
    Suffix varchar(255) NOT NULL
);

CREATE TABLE Conversations (
    ConvoId SERIAL PRIMARY KEY,
    ConvoPubId varchar(255) NOT NULL,
    CreatedAt TIMESTAMPTZ DEFAULT now() NOT NULL,
    UpdatedAt TIMESTAMPTZ DEFAULT now() NOT NULL
);

CREATE TABLE Users_Conversations (
    UserId int REFERENCES Users (UserId) ON UPDATE CASCADE ON DELETE CASCADE,
    ConvoId int REFERENCES Conversations (ConvoId) ON UPDATE CASCADE,
    CONSTRAINT Users_Conversations_id PRIMARY KEY (UserId, ConvoId)
);


CREATE TABLE Messages (
    Id SERIAL NOT NULL ,
    PubId varchar(255) NOT NULL,
    CreatedAt TIMESTAMPTZ DEFAULT now() NOT NULL,
    UpdatedAt TIMESTAMPTZ DEFAULT now() NOT NULL
    -- fk to user
    -- fk to conversations
);
