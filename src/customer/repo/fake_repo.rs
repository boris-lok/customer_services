#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::Mutex;

#[cfg(test)]
use common::utils::alias::AppResult;

#[cfg(test)]
use crate::customer::json::customer::Customer;
#[cfg(test)]
use crate::pb::CreateCustomerRequest;

#[cfg(test)]
use super::repo::CustomerRepo;

#[cfg(test)]
lazy_static::lazy_static! {
    static ref CUSTOMERS: Mutex<HashMap<i64, Customer>> = Mutex::new(HashMap::new());
}

#[cfg(test)]
#[derive(Debug, Default)]
pub struct FakeCustomerRepo {}


#[cfg(test)]
#[async_trait::async_trait]
impl CustomerRepo for FakeCustomerRepo {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>> {
        let customers = CUSTOMERS.lock().unwrap();
        Ok(customers.get(&id).cloned())
    }

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer> {
        let mut customers = CUSTOMERS.lock().unwrap();
        let id = 1;
        let c = Customer {
            id,
            name: request.name.clone(),
            email: request.email.clone(),
            phone: request.phone,
            created_at: chrono::Utc::now(),
            updated_at: None
        };
        customers.insert(id, c.clone());
        Ok(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn it_can_create_a_customer() {
        let r = CreateCustomerRequest {
            name: "boris".to_string(),
            email: None,
            phone: None
        };
        let repo = FakeCustomerRepo::default();
        let response = repo.create(r.clone()).await;

        assert!(response.is_ok());

        if let Ok(response) = response {
            assert_eq!(response.name, r.name)
        }
    }

    #[tokio::test]
    async fn it_can_retrieve_a_customer_by_id() {
        let r = CreateCustomerRequest {
            name: "boris".to_string(),
            email: None,
            phone: None
        };
        let repo = FakeCustomerRepo::default();
        let response = repo.create(r.clone()).await;

        assert!(response.is_ok());

        if let Ok(response) = response {
            let c = repo.get(response.id).await;
            assert!(c.is_ok());
            if let Ok(Some(c)) = c {
                assert_eq!(c.name, r.name);
            } else {
                panic!("failed! Can't retrieve a customer by id.")
            }
        }

    }
}
