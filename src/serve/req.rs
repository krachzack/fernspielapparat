use crate::books::{compile, spec::Book as BookSpec, Book};
use crate::result::Result;
use crate::senses::Input;

use failure::format_err;
use serde::Deserialize;
use serde_yaml::from_str;

/// A request of a controlling application sent over web socket,
/// indicating to the runtime what action to perform.
///
/// It can be converted from YAML with its `decode` method.
#[derive(Debug)]
pub enum Request {
    /// Terminate the currently running phonebook and load the
    /// compiled phonebook received from the client.
    Run(Book),
    /// Keep the current phonebook but start over from the initial state,
    /// and revert all state to initial values, e.g. set playback positions
    /// to the start offset.
    Reset,
    /// A remote request to dial a sequence of inputs.
    Dial(Vec<Input>),
}

/// A raw request after decoding it from YAML.
/// Needs to be compiled before use.
#[derive(Debug, Deserialize)]
#[serde(tag = "invoke", content = "with")]
enum Spec {
    #[serde(rename = "run")]
    Run(BookSpec),
    #[serde(rename = "reset")]
    Reset,
    /// 0-9 mean numeric input.
    /// h is hanging up.
    /// p is picking up.
    /// All other characters are ignored.
    #[serde(rename = "dial")]
    Dial(String),
}

impl Request {
    /// Decodes a YAML string into a request.
    ///
    /// If it is a run request
    pub fn decode<S: AsRef<str>>(yaml_source: S) -> Result<Self> {
        from_str(yaml_source.as_ref())
            .map_err(|e| format_err!("malformed fernspielctl request: {}", e))
            .and_then(Spec::compile)
    }
}

impl Spec {
    fn compile(self) -> Result<Request> {
        Ok(match self {
            Spec::Run(string) => Request::Run(compile(string)?),
            Spec::Reset => Request::Reset,
            Spec::Dial(seq) => Request::Dial(
                seq.chars()
                    .filter_map(|c| match c {
                        num @ '0'..='9' => Some(
                            // unwrap is safe, always in range as proven by pattern
                            Input::digit((num as i32) - ('0' as i32)).unwrap(),
                        ),
                        'h' => Some(Input::hang_up()),
                        'p' => Some(Input::pick_up()),
                        _ => None,
                    })
                    .collect(),
            ),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode_run() {
        // given
        let reset = "{
            \"invoke\":\"run\",
            \"with\": {
                \"initial\": \"lonelystate\",
                \"states\":{
                    \"lonelystate\":{}
                },
                \"transitions\":{}
            }
        }";

        // when
        let decoded = Request::decode(reset).expect("failed to decode run request");

        // then
        match decoded {
            Request::Run(book) => assert_eq!(book.states().len(), 1),
            other => panic!("Unexpected request type: {:?}", other),
        }
    }

    #[test]
    fn decode_reset() {
        // given
        let reset = "{
            \"invoke\":\"reset\"
        }";

        // when
        let decoded = Request::decode(reset).expect("failed to decode reset request");

        // then
        match decoded {
            Request::Reset => (),
            other => panic!("Unexpected request type: {:?}", other),
        }
    }

    #[test]
    fn decode_9_hang_up() {
        // given
        let reset = "{
            \"invoke\":\"dial\",
            \"with\": \"9 \t\nh\"
        }";

        // when
        let decoded = Request::decode(reset).expect("failed to decode reset request");

        // then
        match decoded {
            Request::Dial(syms) => {
                assert_eq!(syms, vec![Input::digit(9).unwrap(), Input::hang_up()])
            }
            other => panic!("Unexpected request type: {:?}", other),
        }
    }
}
