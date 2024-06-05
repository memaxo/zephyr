use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, FutureExt, Ready};

const RATE_LIMIT_WINDOW: Duration = Duration::from_secs(60); // Rate limiting window (1 minute)
const MAX_REQUESTS_PER_WINDOW: usize = 100; // Maximum requests per window

struct RateLimiter {
    client_buckets: Arc<Mutex<HashMap<String, ClientBucket>>>,
}

impl RateLimiter {
    fn new() -> Self {
        RateLimiter {
            client_buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_client_bucket(&self, client_id: &str) -> ClientBucket {
        let mut buckets = self.client_buckets.lock().unwrap();
        buckets
            .entry(client_id.to_string())
            .or_insert_with(|| ClientBucket {
                tokens: MAX_REQUESTS_PER_WINDOW,
                last_refill: Instant::now(),
            })
            .clone()
    }

    fn check_rate_limit(&self, client_id: &str) -> Result<(), Error> {
        let mut bucket = self.get_client_bucket(client_id);
        let now = Instant::now();

        if now > bucket.last_refill + RATE_LIMIT_WINDOW {
            bucket.tokens = MAX_REQUESTS_PER_WINDOW;
            bucket.last_refill = now;
        } else {
            let elapsed = now - bucket.last_refill;
            let tokens_to_add = (elapsed.as_secs() * MAX_REQUESTS_PER_WINDOW as u64 / RATE_LIMIT_WINDOW.as_secs() as u64) as usize;
            bucket.tokens = std::cmp::min(bucket.tokens + tokens_to_add, MAX_REQUESTS_PER_WINDOW);
        }

        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            let mut buckets = self.client_buckets.lock().unwrap();
            buckets.insert(client_id.to_string(), bucket);
            Ok(())
        } else {
            Err(Error::from(actix_web::error::ErrorTooManyRequests("Rate limit exceeded")))
        }
    }
}

struct ClientBucket {
    tokens: usize,
    last_refill: Instant,
}

pub struct RateLimiterMiddleware {
    rate_limiter: RateLimiter,
}

impl RateLimiterMiddleware {
    pub fn new() -> Self {
        RateLimiterMiddleware {
            rate_limiter: RateLimiter::new(),
        }
    }
}

impl<S, B> Transform<S> for RateLimiterMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimiterMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RateLimiterMiddlewareMiddleware {
            service,
            rate_limiter: self.rate_limiter.clone(),
        })
    }
}

pub struct RateLimiterMiddlewareMiddleware<S> {
    service: S,
    rate_limiter: RateLimiter,
}

impl<S, B> Service for RateLimiterMiddlewareMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let client_id = req
            .headers()
            .get("X-Client-ID")
            .and_then(|client_id| client_id.to_str().ok())
            .unwrap_or("default");

        if let Err(err) = self.rate_limiter.check_rate_limit(client_id) {
            return ok(req.into_response(err));
        }

        ok(self.service.call(req))
    }
}