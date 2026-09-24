use crate::models::system::Contact;
use crate::resource::Resource;
use reqwest::Error;
use std::sync::Arc;

pub struct SystemResource {
    resource: Arc<Resource>,
}

impl SystemResource {
    pub fn new(resource: Arc<Resource>) -> Self {
        Self { resource }
    }

    pub async fn list_contacts(&self) -> Result<Vec<Contact>, Error> {
        self.resource.get::<Vec<Contact>>("/v2/contacts").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    #[tokio::test]
    async fn list_contacts_returns_contacts() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/contacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1,
                    "organizationId": 42,
                    "uid": "abc123",
                    "firstName": "John",
                    "lastName": "Doe",
                    "email": "john@example.com",
                    "phone": "+31612345678",
                    "jobTitle": "Developer"
                },
                {
                    "id": 2,
                    "organizationId": 42,
                    "uid": "def456",
                    "firstName": "Jane",
                    "lastName": "Doe",
                    "email": "jane@example.com",
                    "phone": "+31687654321",
                    "jobTitle": "Manager"
                }
            ])))
            .mount(&server)
            .await;

        let resource = Arc::new(Resource::new(server.uri()));
        let system = SystemResource::new(Arc::clone(&resource));

        let contacts = system.list_contacts().await.unwrap();

        assert_eq!(contacts.len(), 2);

        assert_eq!(contacts[0].id, 1);
        assert_eq!(contacts[0].first_name, "John");
        assert_eq!(contacts[0].last_name, "Doe");
        assert_eq!(contacts[0].email, "john@example.com");

        assert_eq!(contacts[1].id, 2);
        assert_eq!(contacts[1].first_name, "Jane");
        assert_eq!(contacts[1].last_name, "Doe");
        assert_eq!(contacts[1].email, "jane@example.com");
    }
}
