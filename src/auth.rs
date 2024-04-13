use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_auth_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 || split[0] != "Basic" {
            return None;
        }
        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(encoded: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(encoded).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let spilt = decoded_str.split(":").collect::<Vec<_>>();

        if spilt.len() != 2 {
            return None;
        }

        Some(BasicAuth {
            username: spilt[0].to_string(),
            password: spilt[1].to_string(),
        })
    }

    fn check_user(auth: &BasicAuth) -> bool {
        auth.username == "Aladdin" && auth.password == "open sesame"
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization").unwrap();
        if let Some(auth) = Self::from_auth_header(auth_header) {
            if Self::check_user(&auth) {
                return Outcome::Success(auth);
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
