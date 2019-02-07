use jsonwebtoken::{encode, decode, Validation, TokenData, Header};
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

 #[derive(Deserialize, Serialize, Debug)]
 pub struct Claims {
     user_id: i32,
     exp: u64,
 }

pub fn encode_token(user_id: i32) -> String {
    let expiry_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(3600);
    let exp = expiry_time.as_secs();
    let claims = Claims{ user_id, exp };
    // TODO: get the secret from config
    encode(&Header::default(), &claims, "secret".as_ref()).unwrap()
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

    #[test]
    fn decode_token() {
        let token = "J0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxLCJleHAiOjE1NDk1NDk2ODV9.vkl8-YAVU0JvZukKkKb7E3DBrbiRalBGsj379sW0_nM";
        let decoded = decode::<Claims>(&token, "secret".as_ref(), &Validation::default());
        if let Err(e) = &decoded {
            println!("decode err: {}", e);
        }

        assert!(decoded.is_ok());
    }
}