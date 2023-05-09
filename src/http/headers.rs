use super::ParseError::{self, InvalidHeaders};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Header<'buf> {
    header: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

#[derive(Debug)]
pub struct Headers<'buf> {
    headers: Vec<Header<'buf>>,
}

impl<'buf> Headers<'buf> {
    const REQUIRED_HEADERS: [(&'static str, &'static str); 1] = [("Accept", "")];

    fn get_header_by_name(&self, name: &'buf str) -> Option<&Header<'buf>> {
        self.headers
            .iter()
            .filter(|&header_obj| header_obj.header.contains_key(name))
            .next()
    }

    pub fn validate_required_headers(&self) -> Result<(), ParseError> {
        for (req_name, exp_value) in Self::REQUIRED_HEADERS {
            if let Some(header_obj) = self.get_header_by_name(req_name) {
                match header_obj
                    .header
                    .values()
                    .any(|value| value.contains(exp_value))
                {
                    true => (),
                    false => return Err(InvalidHeaders),
                }
            } else {
                return Err(InvalidHeaders);
            }
        }
        Ok(())
    }
}

impl<'buf> From<&'buf str> for Headers<'buf> {
    fn from(request: &'buf str) -> Self {
        let headers: Vec<Header> = request
            .split("\r\n")
            .take_while(|&s| s.contains(": "))
            .map(|header| Header::from(header))
            .collect();

        Headers { headers }
    }
}

impl<'buf> From<&'buf str> for Header<'buf> {
    fn from(header: &'buf str) -> Self {
        let mut data = HashMap::new();

        let (key, value) = header.split_once(": ").unwrap();
        let name = key.trim_start_matches("\n");

        for value in value.split(";") {
            data.entry(name)
                .and_modify(|e| match e {
                    Value::Single(prev_value) => {
                        *e = Value::Multiple(vec![prev_value, value]);
                    }
                    Value::Multiple(values) => values.push(value),
                })
                .or_insert(Value::Single(value));
        }

        Self { header: data }
    }
}

impl<'buf> Value<'buf> {
    pub fn contains(&self, exp_value: &'buf str) -> bool {
        match self {
            Self::Single(value) => value.contains(&exp_value),
            Self::Multiple(values) => values.iter().any(|&value| value.contains(&exp_value)),
        }
    }
}
