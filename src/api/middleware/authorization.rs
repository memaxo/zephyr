use std::collections::HashSet;

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, FutureExt, Ready};

use crate::api::middleware::authentication::Claims;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    ReadBlocks,
    WriteBlocks,
    ReadTransactions,
    WriteTransactions,
    // Add more permissions as needed
}

pub struct AuthorizationMiddleware {
    permissions: HashSet<(Method, Path, Permission)>,
}

impl AuthorizationMiddleware {
    pub fn new() -> Self {
        let mut permissions = HashSet::new();
        permissions.insert((Method::Get, Path::BlockchainBlocks, Permission::ReadBlocks));
        permissions.insert((Method::Post, Path::BlockchainBlocks, Permission::WriteBlocks));
        permissions.insert((Method::Get, Path::BlockchainTransactions, Permission::ReadTransactions));
        permissions.insert((Method::Post, Path::BlockchainTransactions, Permission::WriteTransactions));
        // Add more permission mappings as needed

        Self { permissions }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Method {
    Get,
    Post,
    // Add more methods as needed
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Path {
    BlockchainBlocks,
    BlockchainTransactions,
    // Add more paths as needed
}

impl<S, B> Transform<S> for AuthorizationMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthorizationMiddlewareMiddleware {
            service,
            permissions: self.permissions.clone(),
        })
    }
}

pub struct AuthorizationMiddlewareMiddleware<S> {
    service: S,
    permissions: HashSet<(Method, Path, Permission)>,
}

impl<S, B> Service for AuthorizationMiddlewareMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = match req.method().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            // Add more methods as needed
            _ => return ok(req.into_response(Error::from(actix_web::error::ErrorUnauthorized("Invalid method")))),
        };

        let path = match req.path() {
            "/api/blockchain/blocks" => Path::BlockchainBlocks,
            "/api/blockchain/transactions" => Path::BlockchainTransactions,
            // Add more paths as needed
            _ => return ok(req.into_response(Error::from(actix_web::error::ErrorUnauthorized("Invalid path")))),
        };

        let claims = match req.extensions().get::<Claims>() {
            Some(claims) => claims.clone(),
            None => return ok(req.into_response(Error::from(actix_web::error::ErrorUnauthorized("Missing claims")))),
        };

        let required_permissions = claims
            .roles
            .iter()
            .flat_map(|role| match role {
                Role::Admin => self.permissions.iter().map(|(_, _, permission)| permission),
                Role::User => self
                    .permissions
                    .iter()
                    .filter(|(permission_method, permission_path, _)| {
                        *permission_method == method && *permission_path == path
                    })
                    .map(|(_, _, permission)| permission),
            })
            .collect::<HashSet<_>>();

        let has_permission = required_permissions.contains(&Permission::ReadBlocks)
            || required_permissions.contains(&Permission::WriteBlocks)
            || required_permissions.contains(&Permission::ReadTransactions)
            || required_permissions.contains(&Permission::WriteTransactions);

        if has_permission {
            ok(self.service.call(req))
        } else {
            ok(req.into_response(Error::from(actix_web::error::ErrorUnauthorized("Insufficient permissions"))))
        }
    }
}