use jsonwebtoken::{encode, decode, Validation, TokenData, Header};
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};
use http::HeaderMap;

// TODO: get the secret from config
const secret: &str = "secret";

 #[derive(Deserialize, Serialize, Debug)]
 pub struct Claims {
     sub: i32,
     exp: u64,
 }

fn validation() -> Validation {
    Validation::default()
}

pub fn encode_token(sub: i32) -> String {
    let expiry_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(3600);
    let exp = expiry_time.as_secs();
    let claims = Claims{ sub, exp };
    encode(&Header::default(), &claims, secret.as_ref()).unwrap()
}

pub fn extract_token(headers: &HeaderMap) -> Option<&str> {
            match headers.get("Authorization") {
                Some(h) => match h.to_str() {
                    Ok(hx) => {
                        debug!("auth header: {}", hx);
                        hx.split(" ").nth(1)
                    },
                    _ => None,
                },
                _ => None,
            }
}

pub fn extract_claims(headers: &HeaderMap) -> Option<Claims> {
    extract_token(headers)
        .and_then(|token| {
            let decoded = decode::<Claims>(&token, secret.as_ref(), &validation());
            if let Err(e) = &decoded {
                debug!("Failed to decode token {}", e);
            }
            decoded.map(|token_data| token_data.claims).ok()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_token() {
        let token = encode_token(123);
        let decoded = decode::<Claims>(&token, "secret".as_ref(), &Validation::default());
        if let Err(e) = &decoded {
            println!("decode err: {}", e);
        }

        assert!(decoded.is_ok());
    }
}