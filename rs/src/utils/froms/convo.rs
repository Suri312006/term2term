use std::vec;

use crate::{
    files::ConfigConvo,
    grpc::{Convo, Participants},
};

impl TryFrom<Convo> for ConfigConvo {
    type Error = &'static str;
    fn try_from(value: Convo) -> std::result::Result<Self, Self::Error> {
        let mut config_users = vec![];
        let _ = value
            .participants
            .ok_or("unable to parse participants from conversation")?
            .users
            .into_iter()
            .map(|user| config_users.push(user.into()));

        Ok(ConfigConvo {
            id: value.id,
            participants: Some(config_users),
            created_at: value.created_at,
        })
    }
}

impl TryFrom<ConfigConvo> for Convo {
    type Error = &'static str;
    fn try_from(value: ConfigConvo) -> std::result::Result<Self, Self::Error> {
        let mut users = vec![];
        let _ = value
            .participants
            .ok_or("unable to parse participants from conversation")?
            .into_iter()
            .map(|user| users.push(user.into()));

        Ok(Convo {
            id: value.id,
            participants: Some(Participants { users }),
            created_at: value.created_at,
        })
    }
}
