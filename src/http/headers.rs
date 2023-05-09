use super::ParseError::{self, InvalidHeaders};

#[derive(Debug)]
pub struct Header<'buf> {
    name: &'buf str,
    value: &'buf str,
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
            .filter(|&header| header.name == name)
            .next()
    }

    pub fn validate_required_headers(&self) -> Result<(), ParseError> {
        for (req_name, exp_value) in Self::REQUIRED_HEADERS {
            if let Some(header) = self.get_header_by_name(req_name) {
                match header.value.contains(exp_value) {
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
        let (key, value) = header.split_once(": ").unwrap();
        let name = key.trim_start_matches("\n");
        Self { name, value }
    }
}
