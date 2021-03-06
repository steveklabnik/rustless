
use serialize::json::{JsonObject, ToJson};
use regex::{Regex, Captures};

static MATCHER: Regex = regex!(r":([a-z][a-z_]*)");

pub struct Path {
    regex: Regex,
    pub path: String,
    pub params: Vec<String>
}

impl Path {

    pub fn apply_captures(&self, jobj: &mut JsonObject, captures: Captures) {
        for param in self.params.iter() {
            jobj.insert(param.clone(), captures.name(param.as_slice()).to_string().to_json());
        }
    }

    pub fn is_match<'a>(&'a self, path: &'a str) -> Option<Captures> {
        match self.regex.captures(path) {
            Some(captures) => Some(captures),
            None => None
        }
    }

    pub fn parse(path: &str, endpoint: bool) -> Result<Path,String> {
        let mut regex_body = "^/".to_string() + Path::sub_regex(path);
        
        if endpoint {
            regex_body = regex_body + "$";
        }

        let regex = match Regex::new(regex_body.as_slice()) {
            Ok(re) => re,
            Err(err) => return Err(format!("{}", err))
        };

        let mut params = vec![];
        for capture in MATCHER.captures_iter(path) {
            params.push(capture.at(1).to_string()); 
        }

        Ok(Path {
            path: path.to_string(),
            params: params,
            regex: regex
        })
    }

    fn sub_regex(path: &str) -> String {
        return MATCHER.replace_all(path, "(?P<$1>[^/?&]+)");
    }

}

#[test]
fn sub_regex() {
    let res = Path::sub_regex(":user_id/messages/:message_id");
    assert_eq!(res.as_slice(), "(?P<user_id>[^/?&]+)/messages/(?P<message_id>[^/?&]+)")
}

#[test]
fn parse_and_match() {
    let path = Path::parse(":user_id/messages/:message_id", true).unwrap();
    assert!(match path.is_match("/1920/messages/100500") {
        Some(captures) => {
            captures.name("user_id") == "1920" && 
            captures.name("message_id") == "100500"
        }
        None => false   
    });
    assert!(match path.is_match("/1920/messages/not_match/100500") {
        Some(_) => false,
        None => true
    });
}

#[test]
fn parse_and_match_root() {
    let path = Path::parse("", true).unwrap();
    assert!(match path.is_match("/") {
        Some(captures) => captures.at(0) == "/",
        None => false
    });
}

#[test]
fn parse_and_match_single_val() {
    let path = Path::parse(":id", true).unwrap();
    assert!(match path.is_match("/550e8400-e29b-41d4-a716-446655440000") {
        Some(captures) => captures.name("id") == "550e8400-e29b-41d4-a716-446655440000",
        None => false
    });
}