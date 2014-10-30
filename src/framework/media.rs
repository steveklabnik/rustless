
use regex::Regex;
use hyper::mime::{Mime, Application, Json, Text, Plain, SubExt};

static MEDIA_REGEX: Regex = regex!(r"vnd\.(?P<vendor>[a-zA-Z_-]+)(?:\.(?P<version>[a-zA-Z0-9]+)(?:\.(?P<param>[a-zA-Z0-9]+))?)?(?:\+(?P<format>[a-zA-Z0-9]+))?");

fn present_or_none(string: String) -> Option<String> {
    if string.is_empty() {
        None
    } else {
        Some(string)
    }
}

#[deriving(Show)]
pub enum Format {
    JsonFormat,
    PlainTextFormat,
    OtherFormat(Mime)
}

impl Format {
    pub fn from_mime(mime: &Mime) -> Format {
        match mime {
            &Mime(Text, Plain, _) => PlainTextFormat,
            &Mime(Application, Json, _) => JsonFormat,
            _ => OtherFormat(mime.clone())
        }
    }
}

pub struct Media {
    pub vendor: String,
    pub version: Option<String>,
    pub param: Option<String>,
    pub format: Format
}

impl Media {

    pub fn default() -> Media {
        Media::from_mime(&Mime(Text, Plain, vec![]))
    }

    pub fn from_mime(mime: &Mime) -> Media {
        Media {
            vendor: "default".to_string(),
            version: None,
            param: None,
            format: Format::from_mime(mime)
        }
    }

    pub fn from_vendor(mime: &Mime) -> Option<Media> {
        match mime {
            &Mime(Application, SubExt(ref ext), _) => {
                match MEDIA_REGEX.captures(ext.as_slice()) {
                    Some(captures) => {
                        let vendor = captures.name("vendor").to_string();
                        let version = present_or_none(captures.name("version").to_string());
                        let param = present_or_none(captures.name("param").to_string());
                        let format_str = present_or_none(captures.name("format").to_string());

                        let format = match format_str {
                            Some(format) => if format.as_slice() == "json" { JsonFormat }
                                            else if format.as_slice() == "txt" { PlainTextFormat }
                                            else { Format::from_mime(mime) },
                            None => Format::from_mime(mime)
                        };

                        Some(Media {
                            vendor: vendor,
                            version: version,
                            param: param,
                            format: format
                        })
                    },
                    None => None
                }
            }
            _ => None
        }
    }
}

#[test]
fn asset_regexp() {
    let captures = MEDIA_REGEX.captures("application/vnd.github.v3.raw+json").unwrap();
    assert_eq!(captures.name("vendor"), "github");
    assert_eq!(captures.name("version"), "v3");
    assert_eq!(captures.name("param"), "raw");
    assert_eq!(captures.name("format"), "json");
    
    let captures = MEDIA_REGEX.captures("application/vnd.github.v3+json").unwrap();
    assert_eq!(captures.name("vendor"), "github");
    assert_eq!(captures.name("version"), "v3");
    assert_eq!(captures.name("param"), "");
    assert_eq!(captures.name("format"), "json");
    
    let captures = MEDIA_REGEX.captures("application/vnd.github+json").unwrap();
    assert_eq!(captures.name("vendor"), "github");
    assert_eq!(captures.name("version"), "");
    assert_eq!(captures.name("param"), "");
    assert_eq!(captures.name("format"), "json");
    
    let captures = MEDIA_REGEX.captures("application/vnd.github").unwrap();
    assert_eq!(captures.name("vendor"), "github");
    assert_eq!(captures.name("version"), "");
    assert_eq!(captures.name("param"), "");
    assert_eq!(captures.name("format"), "");
    
    let captures = MEDIA_REGEX.captures("application/vnd");
    assert!(captures.is_none());
}

#[test]
fn assert_media() {

    match Media::from_mime(&from_str("application/json").unwrap()).format {
        JsonFormat => (),
        _ => panic!("Wrong format")
    }

    match Media::from_mime(&from_str("text/plain").unwrap()).format {
        PlainTextFormat => (),
        _ => panic!("Wrong format")
    }

    match Media::from_mime(&from_str("application/octet-stream").unwrap()).format {
        OtherFormat(_) => (),
        _ => panic!("Wrong format")
    }

}
