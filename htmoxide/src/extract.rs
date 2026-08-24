use crate::{body::Body, error::HtmoxideError};
use axum::{
    body::Body as AxumBody,
    extract::{FromRequest, FromRequestParts},
    http::Request,
};
use std::future::Future;

pub async fn extract_parts<T>(
    parts: &mut axum::http::request::Parts,
    state: &(),
) -> Result<T, HtmoxideError>
where
    T: axum::extract::FromRequestParts<()>,
{
    T::from_request_parts(parts, state)
        .await
        .map_err(HtmoxideError::from_rejection)
}

pub trait LastExtract: Sized {
    type Output;

    fn last_extract(
        req: Request<AxumBody>,
    ) -> impl Future<Output = Result<Self::Output, HtmoxideError>> + Send;
}

impl<T> LastExtract for T
where
    T: FromRequestParts<()>,
{
    type Output = T;

    async fn last_extract(req: Request<AxumBody>) -> Result<Self::Output, HtmoxideError> {
        let (mut parts, _) = req.into_parts();

        T::from_request_parts(&mut parts, &())
            .await
            .map_err(HtmoxideError::from_rejection)
    }
}

impl<T> LastExtract for Body<T>
where
    T: FromRequest<()>,
{
    type Output = Self;

    async fn last_extract(req: Request<AxumBody>) -> Result<Self::Output, HtmoxideError> {
        let value = T::from_request(req, &())
            .await
            .map_err(HtmoxideError::from_rejection)?;

        Ok(Body(value))
    }
}
