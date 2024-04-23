use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::auth::{
    api_key::ApiKeyData,
    auth0::Auth0Data,
    claims::{AccessLevel, Claims},
    error::ClientError,
};
use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::Uri,
    Error, FromRequest,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use awc::Client;
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet, RSAKeyParameters},
    Algorithm, DecodingKey, Validation,
};

const X_API_KEY: &str = "x-api-key";

pub struct AuthMiddleware {
    api_key_data: Rc<ApiKeyData>,
    auth0_data: Rc<Auth0Data>,
    access_level: Rc<AccessLevel>,
}

impl AuthMiddleware {
    pub fn new(auth_data: ApiKeyData, auth0_data: Auth0Data, access_level: AccessLevel) -> Self {
        AuthMiddleware {
            api_key_data: Rc::new(auth_data),
            auth0_data: Rc::new(auth0_data),
            access_level: Rc::new(access_level),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static, // update here
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareFactory<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareFactory {
            access_level: self.access_level.clone(),
            api_key_data: self.api_key_data.clone(),
            auth0_data: self.auth0_data.clone(),
            service: Rc::new(service), // convert S to Rc<S>
        }))
    }
}

pub struct AuthMiddlewareFactory<S> {
    // service: S,
    service: Rc<S>,
    api_key_data: Rc<ApiKeyData>,
    auth0_data: Rc<Auth0Data>,
    access_level: Rc<AccessLevel>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareFactory<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static, // update here
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone the service to keep reference after moving into async block
        let service = Rc::clone(&self.service);
        let api_key_data = self.api_key_data.clone();
        let auth0_data = self.auth0_data.clone();
        let access_level = self.access_level.clone();

        let extractor = BearerAuth::extract(req.request());

        Box::pin(async move {
            let api_key_header = req.headers().get(X_API_KEY);

            // API KEY authentication has ADMIN rights and pass-through Auth0 authentication/authorization
            if api_key_header.is_none() || api_key_header.unwrap().clone() != api_key_data.api_key {
                // Using map_err and question mark will propagate errors
                let credentials = extractor.await.map_err(ClientError::Authentication)?;
                let token = credentials.token();
                let header = decode_header(token).map_err(ClientError::Decode)?;
                let kid = header.kid.ok_or_else(|| {
                    ClientError::NotFound("kid not found in token header".to_string())
                })?;
                let jwks: JwkSet = Client::new()
                    .get(
                        Uri::builder()
                            .scheme("https")
                            .authority(auth0_data.domain.to_string())
                            .path_and_query("/.well-known/jwks.json")
                            .build()
                            .unwrap(),
                    )
                    .send()
                    .await
                    .map_err(ClientError::SendRequestError)?
                    .json()
                    .await
                    .map_err(ClientError::JsonPayloadError)?;
                let jwk = jwks
                    .find(&kid)
                    .ok_or_else(|| ClientError::NotFound("No JWK found for kid".to_string()))?;

                let rsa = match jwk.clone().algorithm {
                    AlgorithmParameters::RSA(rsa) => Ok::<RSAKeyParameters, Error>(rsa),
                    algorithm => Err(ClientError::UnsupportedAlgortithm(algorithm).into()),
                }?;

                let mut validation = Validation::new(Algorithm::RS256);
                validation.set_audience(&[auth0_data.audience.to_string()]);
                validation.set_issuer(&[Uri::builder()
                    .scheme("https")
                    .authority(auth0_data.domain.to_string())
                    .path_and_query("/")
                    .build()
                    .unwrap()]);
                let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                    .map_err(ClientError::Decode)?;
                let token =
                    decode::<Claims>(token, &key, &validation).map_err(ClientError::Decode)?;
                // In this part, we are going to validate, if the user has the right permissions to access this endpoint
                // be sure, to update your Auth0 API, to include permissions on your JWT access token
                match token.claims.validate_permissions(access_level.to_string()) {
                    false => Err(ClientError::NoPermission(access_level.to_string())),
                    true => Ok(()),
                }?;
            }

            // Continue with the next middleware / handler
            let res = service.call(req).await?;
            // Map to L type
            Ok(res.map_into_left_body())
        })
    }
}

// async fn get_some_data() -> String {
//     "Data".into()
// }
