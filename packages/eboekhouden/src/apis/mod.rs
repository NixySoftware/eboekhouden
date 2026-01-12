use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_string())
        }
    }
}

pub mod administration_api;
pub mod cost_center_api;
pub mod email_template_api;
pub mod invoice_api;
pub mod invoice_template_api;
pub mod ledger_api;
pub mod member_api;
pub mod mutation_api;
pub mod product_api;
pub mod relation_api;
pub mod session_api;
pub mod unit_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn administration_api(&self) -> &dyn administration_api::AdministrationApi;
    fn cost_center_api(&self) -> &dyn cost_center_api::CostCenterApi;
    fn email_template_api(&self) -> &dyn email_template_api::EmailTemplateApi;
    fn invoice_api(&self) -> &dyn invoice_api::InvoiceApi;
    fn invoice_template_api(&self) -> &dyn invoice_template_api::InvoiceTemplateApi;
    fn ledger_api(&self) -> &dyn ledger_api::LedgerApi;
    fn member_api(&self) -> &dyn member_api::MemberApi;
    fn mutation_api(&self) -> &dyn mutation_api::MutationApi;
    fn product_api(&self) -> &dyn product_api::ProductApi;
    fn relation_api(&self) -> &dyn relation_api::RelationApi;
    fn session_api(&self) -> &dyn session_api::SessionApi;
    fn unit_api(&self) -> &dyn unit_api::UnitApi;
}

pub struct ApiClient {
    administration_api: Box<dyn administration_api::AdministrationApi>,
    cost_center_api: Box<dyn cost_center_api::CostCenterApi>,
    email_template_api: Box<dyn email_template_api::EmailTemplateApi>,
    invoice_api: Box<dyn invoice_api::InvoiceApi>,
    invoice_template_api: Box<dyn invoice_template_api::InvoiceTemplateApi>,
    ledger_api: Box<dyn ledger_api::LedgerApi>,
    member_api: Box<dyn member_api::MemberApi>,
    mutation_api: Box<dyn mutation_api::MutationApi>,
    product_api: Box<dyn product_api::ProductApi>,
    relation_api: Box<dyn relation_api::RelationApi>,
    session_api: Box<dyn session_api::SessionApi>,
    unit_api: Box<dyn unit_api::UnitApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            administration_api: Box::new(administration_api::AdministrationApiClient::new(
                configuration.clone(),
            )),
            cost_center_api: Box::new(cost_center_api::CostCenterApiClient::new(
                configuration.clone(),
            )),
            email_template_api: Box::new(email_template_api::EmailTemplateApiClient::new(
                configuration.clone(),
            )),
            invoice_api: Box::new(invoice_api::InvoiceApiClient::new(configuration.clone())),
            invoice_template_api: Box::new(invoice_template_api::InvoiceTemplateApiClient::new(
                configuration.clone(),
            )),
            ledger_api: Box::new(ledger_api::LedgerApiClient::new(configuration.clone())),
            member_api: Box::new(member_api::MemberApiClient::new(configuration.clone())),
            mutation_api: Box::new(mutation_api::MutationApiClient::new(configuration.clone())),
            product_api: Box::new(product_api::ProductApiClient::new(configuration.clone())),
            relation_api: Box::new(relation_api::RelationApiClient::new(configuration.clone())),
            session_api: Box::new(session_api::SessionApiClient::new(configuration.clone())),
            unit_api: Box::new(unit_api::UnitApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn administration_api(&self) -> &dyn administration_api::AdministrationApi {
        self.administration_api.as_ref()
    }
    fn cost_center_api(&self) -> &dyn cost_center_api::CostCenterApi {
        self.cost_center_api.as_ref()
    }
    fn email_template_api(&self) -> &dyn email_template_api::EmailTemplateApi {
        self.email_template_api.as_ref()
    }
    fn invoice_api(&self) -> &dyn invoice_api::InvoiceApi {
        self.invoice_api.as_ref()
    }
    fn invoice_template_api(&self) -> &dyn invoice_template_api::InvoiceTemplateApi {
        self.invoice_template_api.as_ref()
    }
    fn ledger_api(&self) -> &dyn ledger_api::LedgerApi {
        self.ledger_api.as_ref()
    }
    fn member_api(&self) -> &dyn member_api::MemberApi {
        self.member_api.as_ref()
    }
    fn mutation_api(&self) -> &dyn mutation_api::MutationApi {
        self.mutation_api.as_ref()
    }
    fn product_api(&self) -> &dyn product_api::ProductApi {
        self.product_api.as_ref()
    }
    fn relation_api(&self) -> &dyn relation_api::RelationApi {
        self.relation_api.as_ref()
    }
    fn session_api(&self) -> &dyn session_api::SessionApi {
        self.session_api.as_ref()
    }
    fn unit_api(&self) -> &dyn unit_api::UnitApi {
        self.unit_api.as_ref()
    }
}
