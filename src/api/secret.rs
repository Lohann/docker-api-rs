//! Secrets are sensitive data that can be used by services. Swarm mode must be enabled for these endpoints to work.

use crate::Result;

pub mod data {
    use crate::api::{Driver, Labels, ObjectVersion};
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "chrono")]
    use chrono::{DateTime, Utc};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SecretInfo {
        #[serde(rename = "ID")]
        pub id: String,
        pub version: ObjectVersion,
        #[cfg(feature = "chrono")]
        pub created_at: DateTime<Utc>,
        #[cfg(not(feature = "chrono"))]
        pub created_at: String,
        #[cfg(feature = "chrono")]
        pub updated_at: DateTime<Utc>,
        #[cfg(not(feature = "chrono"))]
        pub updated_at: String,
        pub spec: SecretSpec,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SecretSpec {
        pub name: String,
        pub labels: Labels,
        pub data: String,
        pub driver: Driver,
        pub templating: Driver,
    }
}

pub mod opts {
    use crate::api::Filter;

    use std::collections::HashMap;

    impl_url_opts_builder!(SecretList);

    pub enum SecretFilter {
        Id(String),
        LabelKey(String),
        LabelKeyVal(String, String),
        Name(String),
        Names(String),
    }

    impl Filter for SecretFilter {
        fn query_key_val(&self) -> (&'static str, String) {
            match &self {
                SecretFilter::Id(id) => ("id", id.to_owned()),
                SecretFilter::LabelKey(label) => ("label", label.to_owned()),
                SecretFilter::LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
                SecretFilter::Name(name) => ("name", name.to_owned()),
                SecretFilter::Names(names) => ("names", names.to_owned()),
            }
        }
    }

    impl SecretListOptsBuilder {
        impl_filter_func!(SecretFilter);
    }
}

pub use data::*;
pub use opts::*;

impl_api_ty!(Secret => name: N);

impl<'docker> Secret<'docker> {
    /// Inspects a secret.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SecretInspect)
    pub async fn inspect(&self) -> Result<SecretInfo> {
        self.docker
            .get_json(&format!("/secrets/{}", self.name)[..])
            .await
    }

    /// Delete a secret.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SecretDelete)
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/secrets/{}", self.name))
            .await
            .map(|_| ())
    }
}

impl<'docker> Secrets<'docker> {
    /// List secrets.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SecretList)
    pub async fn list(&self, opts: &SecretListOpts) -> Result<Vec<SecretInfo>> {
        let mut path = vec!["/secrets".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker
            .get_json::<Vec<SecretInfo>>(&path.join("?"))
            .await
    }
}