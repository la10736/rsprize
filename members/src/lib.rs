use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, Debug)]
pub struct Member {
    id: u32,
    name: String,
    photo: Photo,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
enum Type {
    Member,
}

#[derive(Deserialize, Eq, PartialEq, Debug)]
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
      assert_eq!(Member {id: 140287212, name: "Alessandro Re".to_owned(), photo: Photo {
            kind: Type::Member, 
            photo: "https://secure.meetupstatic.com/photos/member/7/a/a/a/member_188191402.jpeg".to_owned(),
            thumb: "https://secure.meetupstatic.com/photos/member/7/a/a/a/thumb_188191402.jpeg".to_owned(),
         } }, member)
    }
}
