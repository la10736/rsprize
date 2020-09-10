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
}

#[cfg(test)]
mod tests {
   use super::*;
   use pretty_assertions::assert_eq;

   #[test]
   fn happy() {
      let json = r#"{
            "id":140287212,
            "name":"Alessandro Re",
            "photo":{
               "id":188191402,
               "photo_link":"https://secure.meetupstatic.com/photos/member/7/a/a/a/member_188191402.jpeg",
               "thumb_link":"https://secure.meetupstatic.com/photos/member/7/a/a/a/thumb_188191402.jpeg",
               "type":"member",
               "base_url":"https://secure.meetupstatic.com"
            },
            "event_context":{
               "host":false
            }
      }"#;
      
      let member: Member = serde_json::from_str(json).unwrap();
      assert_eq!(Member {id: 140287212, name: "Alessandro Re".to_owned(), photo: Some(Photo {
            kind: Type::Member, 
            photo: "https://secure.meetupstatic.com/photos/member/7/a/a/a/member_188191402.jpeg".to_owned(),
            thumb: "https://secure.meetupstatic.com/photos/member/7/a/a/a/thumb_188191402.jpeg".to_owned(),
         }) }, member)
    }

    #[test]
    fn photo_can_be_optional() {
        let json = r#"{
            "id":267215039,
            "name":"Marco Bianchi",
            "event_context":{
               "host":false
            }
         }"#;
      
      let member: Member = serde_json::from_str(json).unwrap();
      assert_eq!(Member {id: 267215039, name: "Marco Bianchi".to_owned(), photo: None }, member)
    }
}

pub fn read_all(input: impl std::io::Read) -> Result<Vec<Member>, serde_json::Error> {
    #[derive(Deserialize)]
    struct Container {
        member: Member,
    }

    serde_json::from_reader::<_, HashMap<String, Container>>(input)
        .map(|members_map|
            members_map.into_iter().map(|(_k, c)| c.member).collect()
        )
}

pub fn read_accept(input: impl std::io::Read) -> Result<Vec<Member>, serde_json::Error> {
    #[derive(Deserialize)]
    struct Container {
        member: Member,
        rsvp: Rsvp,
    }

    #[derive(Deserialize)]
    struct Rsvp {
        response: String,
    }

    serde_json::from_reader::<_, HashMap<String, Container>>(input)
        .map(|members_map|
            members_map.into_iter()
                .filter_map(
                    |(_k, c)| if c.rsvp.response == "yes" { Some(c.member) } else { None } )
                .collect()
        )
}
