pub fn create() -> (String, String, String) {
    return (
        String::from("username"),
        String::from("dbname"),
        String::from("password"),
    );
}

pub fn destroy(username: &String) {}
