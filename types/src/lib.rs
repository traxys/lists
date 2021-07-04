use serde::{Deserialize, Serialize};
pub use uuid;

#[derive(Serialize, Deserialize, thiserror::Error, Debug)]
#[error("Api returned an error: {description}")]
pub struct RspErr {
    pub code: usize,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RspData<T> {
    Ok(T),
    Err(RspErr),
}

impl<T> From<RspData<T>> for Result<T, RspErr> {
    fn from(v: RspData<T>) -> Self {
        match v {
            RspData::Ok(v) => Ok(v),
            RspData::Err(v) => Err(v),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Empty {}

pub mod login {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub password: String,
        pub username: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub token: String,
    }
}

pub mod create_list {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub id: Uuid,
    }
}

pub mod get_lists {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ListStatus {
        Owned,
        SharedWrite,
        SharedRead,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ListInfo {
        pub id: Uuid,
        pub status: ListStatus,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub results: HashMap<String, ListInfo>,
    }
}

pub mod search_account {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub id: Uuid,
    }
}

pub mod read_list {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Item {
        pub id: i32,
        pub name: String,
        pub amount: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub items: Vec<Item>,
        pub readonly: bool,
    }
}

pub mod add_to_list {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub name: String,
        pub amount: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub id: i32,
    }
}

pub mod share_list {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub share_with: Uuid,
        pub readonly: bool,
    }

    pub type Response = super::Empty;
}

pub mod delete_item {
    pub type Response = super::Empty;
}

pub mod delete_share {
    pub type Response = super::Empty;
}

pub mod delete_list {
    pub type Response = super::Empty;
}

pub mod register {
    pub use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub username: String,
        pub password: String,
    }

    pub type Response = super::Empty;
}
