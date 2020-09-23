use std::{io::BufRead, io::Read, process::Child, process::Command, process::Stdio};

// use members::Member;

fn extract_port(out: impl Read) -> u16 {
    std::io::BufReader::new(out)
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.contains(" service on "))
        .nth(0)
        .and_then(|l| l.rsplit(":").nth(0).and_then(|p| p.parse().ok()))
        .expect("Cannot extract port. Is logging enable?")
}

struct App {
    child: Child,
    port: u16,
}

impl App {
    fn spawn<'a>(envs: impl IntoIterator<Item = (&'a str, &'a str)>) -> Self {
        let mut child = Command::new("cargo")
            .arg("run")
            .args(&["--bin", "webapp"])
            .env("RUST_LOG", "info")
            .envs(envs)
            .stderr(Stdio::piped())
            .spawn()
            .expect("Cannot start webapp");
        let port = extract_port(child.stderr.take().expect("No stderr"));
        App { child, port }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::spawn(vec![("RSPRIZE_PORT", "0")])
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

// #[actix_rt::test]
// async fn health_check_works() {
//     let app = App::default();

//     let client = reqwest::Client::new();

//     let response = client
//         .get(&format!("http://127.0.0.1:{}/health_check", app.port))
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     // Assert
//     assert!(response.status().is_success());
//     assert_eq!(Some(0), response.content_length());
// }

// #[actix_rt::test]
// async fn ask_3_prizes() {
//     let app = App::spawn(vec![("RSPRIZE_PORT", "0"), ("RSPRIZE_SEED", "324563")]);

//     let client = reqwest::Client::new();

//     let body = std::fs::read_to_string("tests/resources/members.json").unwrap();

//     let response = client
//         .get(&format!("http://127.0.0.1:{}/prizes", app.port))
//         .query(&[("n", "3")])
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     // Assert
//     assert!(response.status().is_success());

//     let members: Vec<Member> =
//         serde_json::from_reader(response.text().await.unwrap().as_bytes()).unwrap();

//     assert_eq!(3, members.len());
//     assert_eq!("Nicola Musatti", members[0].name);
//     assert_eq!("Marco Bianchi", members[1].name);
//     assert_eq!("Andrea Rossini", members[2].name);
// }

// #[actix_rt::test]
// async fn just_one_prize_if_no_param() {
//     let app = App::default();

//     let client = reqwest::Client::new();

//     let body = std::fs::read_to_string("tests/resources/members.json").unwrap();

//     let response = client
//         .get(&format!("http://127.0.0.1:{}/prizes", app.port))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to execute request.");

//     // Assert
//     assert!(response.status().is_success());

//     let members: Vec<Member> =
//         serde_json::from_reader(response.text().await.unwrap().as_bytes()).unwrap();

//     assert_eq!(1, members.len());
// }
