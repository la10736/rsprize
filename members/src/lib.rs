use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Member {
    id: u32,
    name: String,
    photo: Option<Photo>,
}

#[derive(Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum Type {
    Member,
}

#[derive(Deserialize, Eq, PartialEq, Debug, Clone)]
struct Photo {
    #[serde(rename = "type")]
    kind: Type,
    #[serde(rename = "photo_link")]
    photo: String,
    #[serde(rename = "thumb_link")]
    thumb: String,
    #[serde(rename = "highres_link")]
    highres: Option<String>,
}

pub fn read_all(input: impl std::io::Read) -> Result<Vec<Member>, serde_json::Error> {
    #[derive(Deserialize)]
    struct Container {
        member: Member,
    }

    serde_json::from_reader::<_, HashMap<String, Container>>(input)
        .map(|members_map| members_map.into_iter().map(|(_k, c)| c.member).collect())
}

pub fn read_accepted_rsvp(input: impl std::io::Read) -> Result<Vec<Member>, serde_json::Error> {
    #[derive(Deserialize)]
    struct Container {
        member: Member,
        rsvp: Rsvp,
    }

    #[derive(Deserialize)]
    struct Rsvp {
        response: String,
    }

    serde_json::from_reader::<_, HashMap<String, Container>>(input).map(|members_map| {
        members_map
            .into_iter()
            .filter_map(|(_k, c)| {
                if c.rsvp.response == "yes" {
                    Some(c.member)
                } else {
                    None
                }
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    mod deserialize {
        use super::{assert_eq, *};

        mod member {
            use super::{assert_eq, *};

            #[test]
            fn with_photo() {
                let json = r#"{
                    "id":144223344,
                    "name":"Alessandro Regina",
                    "photo":{
                        "id":188991122,
                        "photo_link":"https://secure.meetupstatic.com/photos/member/7/a/a/a/member_188991122.jpeg",
                        "thumb_link":"https://secure.meetupstatic.com/photos/member/7/a/a/a/thumb_188991122.jpeg",
                        "highres_link":"https://secure.meetupstatic.com/photos/member/7/a/a/a/highres_188991122.jpeg",
                        "type":"member",
                        "base_url":"https://secure.meetupstatic.com"
                    }
                }"#;

                let member: Member = serde_json::from_str(json).unwrap();
                assert_eq!(Member {id: 144223344, name: "Alessandro Regina".to_owned(), photo: Some(Photo {
                    kind: Type::Member,
                    photo: "https://secure.meetupstatic.com/photos/member/7/a/a/a/member_188991122.jpeg".to_owned(),
                    thumb: "https://secure.meetupstatic.com/photos/member/7/a/a/a/thumb_188991122.jpeg".to_owned(),
                    highres: Some("https://secure.meetupstatic.com/photos/member/7/a/a/a/highres_188991122.jpeg".to_owned()),
                }) }, member)
            }

            #[test]
            fn without_photo() {
                let json = r#"{
                    "id":667788039,
                    "name":"Marco Verdi",
                    "event_context":{
                        "host":false
                    }
                }"#;

                let member: Member = serde_json::from_str(json).unwrap();
                assert_eq!(
                    Member {
                        id: 667788039,
                        name: "Marco Verdi".to_owned(),
                        photo: None
                    },
                    member
                )
            }
        }

        mod members_list {
            use super::{assert_eq, *};

            static DATA: &'static str = r#"
                {
                    "0":{
                        "member":{
                            "id":151237211,
                            "name":"Pippo",
                            "photo":{
                                "id":299293183,
                                "photo_link":"https://secure.meetupstatic.com/photos/member/7/a/a/a/member_299293183.jpeg",
                                "thumb_link":"https://secure.meetupstatic.com/photos/member/7/a/a/a/thumb_299293183.jpeg",
                                "type":"member",
                                "base_url":"https://secure.meetupstatic.com"
                            }
                        },
                        "rsvp":{
                            "id":1135172528,
                            "response":"yes",
                            "guests":0,
                            "updated":1581673584000
                        }
                    },
                    "1":{
                        "member":{
                            "id":222333444,
                            "name":"Paperino",
                            "photo":{
                                "id":555444333,
                                "highres_link":"https://secure.meetupstatic.com/photos/member/d/e/8/f/highres_555444333.jpeg",
                                "photo_link":"https://secure.meetupstatic.com/photos/member/d/e/8/f/member_555444333.jpeg",
                                "thumb_link":"https://secure.meetupstatic.com/photos/member/d/e/8/f/thumb_555444333.jpeg",
                                "type":"member",
                                "base_url":"https://secure.meetupstatic.com"
                            }
                        },
                        "rsvp":{
                            "id":1829333222,
                            "response":"no",
                            "guests":0,
                            "updated":1581689292000
                        }
                    }
                }
                "#;

            #[test]
            fn extract_all() {
                let members = read_all(DATA.as_bytes()).unwrap();

                assert_eq!(2, members.len());
            }

            #[test]
            fn extract_just_accepted_rsvp() {
                let members = read_accepted_rsvp(DATA.as_bytes()).unwrap();

                assert_eq!(1, members.len());
            }
        }
    }
}
