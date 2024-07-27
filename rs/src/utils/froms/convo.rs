use std::vec;

use crate::{
    files::ConfigConvo,
    grpc::{Convo, Participants},
};

impl TryFrom<Convo> for ConfigConvo {
    type Error = &'static str;
    fn try_from(value: Convo) -> std::result::Result<Self, Self::Error> {
        let mut config_users = vec![];

        for user in value
            .participants
            .ok_or("unable to parse participants from conversation")?
            .users
        {
            config_users.push(user.into())
        }

        Ok(ConfigConvo {
            id: value.id,
            participants: Some(config_users),
            created_at: value.created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        files::ConfigConvo,
        grpc::{Convo, Participants, User},
    };

    #[test]
    fn convo_conversions() {
        let convo = Convo {
            id: "id".to_string(),
            participants: Some(Participants {
                users: vec![
                    User {
                        id: "lmao".to_string(),
                        name: "xd".to_string(),
                    },
                    User {
                        id: "lmao".to_string(),
                        name: "xd".to_string(),
                    },
                ],
            }),
            created_at: 123456,
        };

        let config_convo: ConfigConvo = convo.clone().try_into().unwrap();

        let convo2: Convo = config_convo.try_into().unwrap();

        assert!(convo2 == convo);
    }
}

impl TryFrom<ConfigConvo> for Convo {
    type Error = &'static str;
    fn try_from(value: ConfigConvo) -> std::result::Result<Self, Self::Error> {
        let mut users = vec![];

        for user in value
            .participants
            .ok_or("Unable to parse participants from conversation")?
            .into_iter()
        {
            users.push(user.into());
        }

        Ok(Convo {
            id: value.id,
            participants: Some(Participants { users }),
            created_at: value.created_at,
        })
    }
}
