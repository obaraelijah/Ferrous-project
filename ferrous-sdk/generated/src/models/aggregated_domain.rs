/*
 * ferrous
 *
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Generated by: https://openapi-generator.tech
 */

/// AggregatedDomain : A domain



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregatedDomain {
    /// Global tags
    #[serde(rename = "global_tags")]
    pub global_tags: Vec<String>,
    /// Tags which are local to the workspace
    #[serde(rename = "local_tags")]
    pub local_tags: Vec<String>,
    /// The domain's uuid
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    /// The domain that was found
    #[serde(rename = "domain")]
    pub domain: String,
    /// A comment to the domain
    #[serde(rename = "comment")]
    pub comment: String,
}

impl AggregatedDomain {
    /// A domain
    pub fn new(global_tags: Vec<String>, local_tags: Vec<String>, uuid: uuid::Uuid, domain: String, comment: String) -> AggregatedDomain {
        AggregatedDomain {
            global_tags,
            local_tags,
            uuid,
            domain,
            comment,
        }
    }
}
