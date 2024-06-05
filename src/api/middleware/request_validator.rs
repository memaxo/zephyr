use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, FutureExt, Ready};
use serde::Deserialize;
use validator::{Validate, ValidationError};

pub struct RequestValidatorMiddleware;

impl<S, B> Transform<S> for RequestValidatorMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestValidatorMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestValidatorMiddlewareMiddleware { service })
    }
}

pub struct RequestValidatorMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service for RequestValidatorMiddlewareMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request_data = match req
            .payload()
            .map_err(|_| Error::from(actix_web::error::ErrorBadRequest("Invalid request body")))
        {
            Ok(data) => data,
            Err(err) => return ok(req.into_response(err)),
        };

        let request_model: Result<RequestModel, _> = match req.method().as_str() {
            "POST" => serde_json::from_ref(request_data.as_ref()).map_err(|_| {
                Error::from(actix_web::error::ErrorBadRequest("Invalid JSON in request body"))
            }),
            _ => Ok(RequestModel::default()),
        };

        let request_model = match request_model {
            Ok(model) => model,
            Err(err) => return ok(req.into_response(err)),
        };

        if let Err(validation_errors) = request_model.validate() {
            let error_messages: Vec<_> = validation_errors
                .iter()
                .map(|err| format!("{}", err))
                .collect();
            let error_response = ValidationErrorResponse {
                errors: error_messages,
            };
            return ok(
                req.into_response(Error::from(actix_web::error::ErrorBadRequest(
                    serde_json::to_string(&error_response).unwrap_or_default(),
                ))),
            );
        }

        ok(self.service.call(req))
    }
}

#[derive(Debug, Deserialize, Validate)]
struct RequestModel {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    name: String,

    #[validate(range(min = 18, message = "Age must be at least 18"))]
    age: u32,

    #[validate(email(message = "Invalid email address"))]
    email: String,

    // Add more fields and validation rules as needed
}

impl Default for RequestModel {
    fn default() -> Self {
        Self {
            name: String::new(),
            age: 0,
            email: String::new(),
        }
    }
}

#[derive(Serialize)]
struct ValidationErrorResponse {
    errors: Vec<String>,
}

impl From<ValidationError> for String {
    fn from(validation_error: ValidationError) -> Self {
        validation_error.message.to_string()
    }
}