//! Types for EPP RGP restore request

use epp_client_macros::*;

use crate::epp::object::{ElementName, EppObject};
use crate::epp::request::domain::update::{DomainChangeInfo, DomainUpdate, DomainUpdateData};
use crate::epp::request::{CommandWithExtension, Extension};
use crate::epp::xml::{EPP_DOMAIN_RGP_EXT_XMLNS, EPP_DOMAIN_XMLNS};
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for a domain rgp restore request command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::{EppDomainRgpRestoreRequest, EppDomainRgpRestoreRequestResponse};
/// use epp_client::epp::generate_client_tr_id;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a config
///     let mut registry: HashMap<String, EppClientConnection> = HashMap::new();
///     registry.insert(
///         "registry_name".to_owned(),
///         EppClientConnection {
///             host: "example.com".to_owned(),
///             port: 700,
///             username: "username".to_owned(),
///             password: "password".to_owned(),
///             ext_uris: None,
///             tls_files: None,
///         },
///     );
///     let config = EppClientConfig { registry };
///
///     // Create an instance of EppClient, passing the config and the registry you want to connect to
///     let mut client = match EppClient::new(&config, "registry_name").await {
///         Ok(client) => client,
///         Err(e) => panic!("Failed to create EppClient: {}",  e)
///     };
///
///     // Create an EppDomainRgpRestoreRequest instance
///     let domain_restore_req = EppDomainRgpRestoreRequest::new(
///         "eppdev.com",
///         generate_client_tr_id(&client).as_str()
///     );
///
///     // send it to the registry and receive a response of type EppDomainRgpRestoreRequestResponse
///     let response = client.transact::<_, EppDomainRgpRestoreRequestResponse>(&domain_restore_req).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppDomainRgpRestoreRequest =
    EppObject<CommandWithExtension<DomainUpdate, RgpRestoreRequest>>;

/// Type corresponding to the &lt;restore&gt; tag for an rgp restore request
#[derive(Serialize, Deserialize, Debug)]
pub struct RgpRestoreRequestData {
    /// The value of the op attribute in the &lt;restore&gt; tag
    pub op: String,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "rgp:update")]
/// Type for EPP XML &lt;check&gt; command for domains
pub struct RgpRestoreRequest {
    /// XML namespace for the RGP restore extension
    #[serde(rename = "xmlns:rgp", alias = "xmlns")]
    xmlns: String,
    /// The object holding the list of domains to be checked
    #[serde(rename = "rgp:restore", alias = "restore")]
    restore: RgpRestoreRequestData,
}

impl EppDomainRgpRestoreRequest {
    /// Creates a new EppObject for domain rgp restore request corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(name: &str, client_tr_id: &str) -> EppDomainRgpRestoreRequest {
        let command = CommandWithExtension::<DomainUpdate, RgpRestoreRequest> {
            command: DomainUpdate {
                domain: DomainUpdateData {
                    xmlns: EPP_DOMAIN_XMLNS.to_string(),
                    name: name.into(),
                    add: None,
                    remove: None,
                    change_info: Some(DomainChangeInfo {
                        registrant: None,
                        auth_info: None,
                    }),
                },
            },
            extension: Some(Extension {
                data: RgpRestoreRequest {
                    xmlns: EPP_DOMAIN_RGP_EXT_XMLNS.to_string(),
                    restore: RgpRestoreRequestData {
                        op: "request".to_string(),
                    },
                },
            }),
            client_tr_id: client_tr_id.into(),
        };

        EppObject::build(command)
    }
}
