use actix_web::{dev::ServiceRequest, Error, FromRequest, HttpResponse, Responder, web};
use actix_service::Service;
use futures::future::{ok, Ready};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct AuthenticatedUser {
    pub user_id: String,
    pub role: String,
}

#[derive(Debug)]
pub enum Role {
    Admin,
    User,
    Guest,
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role {
            "Admin" => Role::Admin,
            "User" => Role::User,
            _ => Role::Guest,
        }
    }
}

impl AuthenticatedUser {
    pub fn has_permission(&self, required_role: Role) -> bool {
        let user_role: Role = self.role.as_str().into();
        match required_role {
            Role::Admin => user_role == Role::Admin,
            Role::User => user_role == Role::Admin || user_role == Role::User,
            Role::Guest => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    sub: String,
    exp: usize,
    token_type: String,
}

lazy_static! {
    static ref ATTEMPT_TRACKER: Mutex<HashMap<String, (u64, u64)>> = Mutex::new(HashMap::new());
    static ref TOKEN_BLACKLIST: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &ServiceRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let ip = req.peer_addr().unwrap().ip().to_string();
        if !rate_limit_check(&ip) {
            return ok(Err(actix_web::error::Error::from(actix_web::http::StatusCode::TOO_MANY_REQUESTS)));
        }

        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .map(|token| token.trim_start_matches("Bearer ").to_string());

        match token {
            Some(token) => match validate_jwt(&token) {
                Ok(auth_user) => ok(Ok(auth_user)),
                Err(e) => ok(Err(e)),
            },
            None => ok(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))),
        }
    }
}

fn rate_limit_check(ip: &str) -> bool {
    let mut attempts = ATTEMPT_TRACKER.lock().unwrap();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let entry = attempts.entry(ip.to_string()).or_insert((0, current_time));

    if current_time - entry.1 < 60 && entry.0 > 5 {
        warn!("Rate limit exceeded for IP: {}", ip);
        return false;
    }

    if current_time - entry.1 >= 60 {
        *entry = (1, current_time);
    } else {
        entry.0 += 1;
    }
    info!("Rate limit check passed for IP: {}", ip);

    true
}

fn validate_jwt(token: &str) -> Result<AuthenticatedUser, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    if is_token_revoked(token) {
        warn!("Token revoked for user: {}", token_data.claims.sub);
        return Err(actix_web::error::ErrorUnauthorized("Token has been revoked"));
    }

    info!("Token validated for user: {}", token_data.claims.sub);
    Ok(AuthenticatedUser {
        user_id: token_data.claims.sub,
        role: token_data.claims.role,
    })
}

fn is_token_revoked(token: &str) -> bool {
    let blacklist = TOKEN_BLACKLIST.read().unwrap();
    blacklist.contains(token)
}

#[derive(Serialize, Deserialize)]
struct RevokeTokenRequest {
    token: String,
}

async fn revoke_token(auth_user: AuthenticatedUser, data: web::Json<RevokeTokenRequest>) -> impl Responder {
    let mut blacklist = TOKEN_BLACKLIST.write().unwrap();
    blacklist.insert(data.token.clone());
    info!("Token revoked successfully for user: {}", auth_user.user_id);
    HttpResponse::Ok().json(serde_json::json!({ "message": "Token revoked successfully" }))
}

fn generate_refresh_token(user_id: &str) -> String {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(7 * 24 * 60 * 60))
        .expect("valid timestamp")
        .duration_since(UNIX_EPOCH)
        .expect("valid duration")
        .as_secs();

    let claims = RefreshTokenClaims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
        token_type: "refresh".to_string(),
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).expect("token encoding")
}

#[derive(Serialize, Deserialize)]
struct RefreshTokenRequest {
    refresh_token: String,
}

async fn refresh_access_token(data: web::Json<RefreshTokenRequest>) -> impl Responder {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<RefreshTokenClaims>(&data.refresh_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    match token_data {
        Ok(data) if data.claims.token_type == "refresh" => {
            let new_access_token = generate_jwt(&data.claims.sub);
            HttpResponse::Ok().json(serde_json::json!({ "access_token": new_access_token }))
        }
        _ => HttpResponse::Unauthorized().finish(),
    }
}

// Add this endpoint to your service configuration
// .service(web::resource("/refresh_access_token").route(web::post().to(refresh_access_token)))