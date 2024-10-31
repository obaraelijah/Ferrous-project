/*
 * ferrous
 *
 * The core component of ferrous-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Generated by: https://openapi-generator.tech
 */

/// AggregatedWorkspace : The aggregated results of a workspace

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregatedWorkspace {
    /// The hosts found by this workspace
    #[serde(rename = "hosts")]
    pub hosts: ::std::collections::HashMap<String, crate::models::AggregatedHost>,
    /// The ports found by this workspace
    #[serde(rename = "ports")]
    pub ports: ::std::collections::HashMap<String, crate::models::AggregatedPort>,
    /// The services found by this workspace
    #[serde(rename = "services")]
    pub services: ::std::collections::HashMap<String, crate::models::AggregatedService>,
    /// The domains found by this workspace
    #[serde(rename = "domains")]
    pub domains: ::std::collections::HashMap<String, crate::models::AggregatedDomain>,
}

impl AggregatedWorkspace {
    /// The aggregated results of a workspace
    pub fn new(
        hosts: ::std::collections::HashMap<String, crate::models::AggregatedHost>,
        ports: ::std::collections::HashMap<String, crate::models::AggregatedPort>,
        services: ::std::collections::HashMap<String, crate::models::AggregatedService>,
        domains: ::std::collections::HashMap<String, crate::models::AggregatedDomain>,
    ) -> AggregatedWorkspace {
        AggregatedWorkspace {
            hosts,
            ports,
            services,
            domains,
        }
    }
}
