//! Types for EPP contact create request

use epp_client_macros::*;

use crate::epp::object::data;
use crate::epp::object::{ElementName, EppObject, StringValue};
use crate::epp::request::Command;
use crate::epp::xml::EPP_CONTACT_XMLNS;
use serde::{Deserialize, Serialize};

/// Type that represents the &lt;epp&gt; request for contact &lt;create&gt; command
///
/// ## Usage
///
/// ```no_run
/// use std::collections::HashMap;
///
/// use epp_client::config::{EppClientConfig, EppClientConnection};
/// use epp_client::EppClient;
/// use epp_client::epp::object::data::{Address, Phone, PostalInfo};
/// use epp_client::epp::{EppContactCreate, EppContactCreateResponse};
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
///     // Create the address, postal_info, voice instances
///     let street = vec!["58", "Orchid Road"];
///     let address = Address::new(&street, "New York", "New York", "392374", "US");
///     let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
///     let mut voice = Phone::new("+1.47237942");
///     voice.set_extension("123");
///     let mut fax = Phone::new("+1.86698799");
///     fax.set_extension("677");
///
///     // Create an EppContactCreate instance
///     let mut contact_create = EppContactCreate::new(
///         "eppdev-contact-100",
///         "contact@eppdev.net",
///         postal_info,
///         voice,
///         "epP4uthd#v",
///         generate_client_tr_id(&client).as_str()
///     );
///     contact_create.set_fax(fax);
///
///     // send it to the registry and receive a response of type EppContactCreateResponse
///     let response = client.transact::<_, EppContactCreateResponse>(&contact_create).await.unwrap();
///
///     println!("{:?}", response);
///
///     client.logout().await.unwrap();
/// }
/// ```
pub type EppContactCreate = EppObject<Command<ContactCreate>>;

/// Type for elements under the contact &lt;create&gt; tag
#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    /// XML namespace for contact commands
    #[serde(rename = "xmlns:contact", alias = "xmlns")]
    xmlns: String,
    /// Contact &lt;id&gt; tag
    #[serde(rename = "contact:id", alias = "id")]
    id: StringValue,
    /// Contact &lt;postalInfo&gt; tag
    #[serde(rename = "contact:postalInfo", alias = "postalInfo")]
    postal_info: data::PostalInfo,
    /// Contact &lt;voice&gt; tag
    #[serde(rename = "contact:voice", alias = "voice")]
    voice: data::Phone,
    /// Contact &lt;fax&gt; tag,
    #[serde(rename = "contact:fax", alias = "fax")]
    fax: Option<data::Phone>,
    /// Contact &lt;email&gt; tag
    #[serde(rename = "contact:email", alias = "email")]
    email: StringValue,
    /// Contact &lt;authInfo&gt; tag
    #[serde(rename = "contact:authInfo", alias = "authInfo")]
    auth_info: data::ContactAuthInfo,
}

#[derive(Serialize, Deserialize, Debug, ElementName)]
#[element_name(name = "create")]
/// Type for EPP XML &lt;create&gt; command for contacts
pub struct ContactCreate {
    /// Data for &lt;create&gt; command for contact
    #[serde(rename = "contact:create", alias = "create")]
    pub contact: Contact,
}

impl EppContactCreate {
    /// Creates a new EppObject for contact create corresponding to the &lt;epp&gt; tag in EPP XML
    pub fn new(
        id: &str,
        email: &str,
        postal_info: data::PostalInfo,
        voice: data::Phone,
        auth_password: &str,
        client_tr_id: &str,
    ) -> EppContactCreate {
        let contact_create = ContactCreate {
            contact: Contact {
                xmlns: EPP_CONTACT_XMLNS.to_string(),
                id: id.into(),
                postal_info,
                voice,
                fax: None,
                email: email.into(),
                auth_info: data::ContactAuthInfo::new(auth_password),
            },
        };

        EppObject::build(Command::<ContactCreate>::new(contact_create, client_tr_id))
    }

    /// Sets the &lt;fax&gt; data for the request
    pub fn set_fax(&mut self, fax: data::Phone) {
        self.data.command.contact.fax = Some(fax);
    }
}
