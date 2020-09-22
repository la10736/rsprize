// use assert_cmd::Command;
// use predicates::{prelude::*, str::contains};

// const SEED: u32 = 324563;

// fn resource(name: &str) -> String {
//     let mut p = std::path::PathBuf::new();
//     p.push("tests");
//     p.push("resources");
//     p.push(name);

//     p.into_os_string().into_string().unwrap()
// }

// #[test]
// fn should_extract_2_prize() {
//     let mut cmd = Command::cargo_bin("extraction").unwrap();

//     let assert = cmd
//         .args(&["-n", "2"])
//         .arg(resource("members.json").as_str())
//         .env("RSPRIZE_SEED", &SEED.to_string())
//         .assert();

//     assert.success().stdout(
//         contains("Nicola Musatti Win!")
//             .and(contains("Marco Bianchi Win!"))
//             .and(contains("Win!").count(2)),
//     );
// }
