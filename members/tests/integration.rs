use members;

#[test]
fn read_members(){
    let f = std::fs::File::open("tests/resources/members.json").unwrap();

    let m = members::read_all(f).unwrap();

    assert_eq!(23, m.len())
}

#[test]
fn read_members_that_accept_event(){
    let f = std::fs::File::open("tests/resources/members.json").unwrap();

    let m = members::read_accept(f).unwrap();

    assert_eq!(15, m.len())
}