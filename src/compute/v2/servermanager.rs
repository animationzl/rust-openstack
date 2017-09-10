// Copyright 2017 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Server management via Compute API.

use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr};

use chrono::{DateTime, FixedOffset};

use super::super::super::{ApiResult, Sort};
use super::super::super::service::Query;
use super::{ComputeV2, protocol};


/// A query to server list.
#[derive(Clone, Debug)]
pub struct ServerQuery<'s> {
    service: &'s ComputeV2,
    /// Underlying query.
    pub query: Query,
}

/// Server manager: working with virtual servers.
///
/// # Examples
///
/// Listing summaries of all servers:
///
/// ```rust,no_run
/// use openstack;
///
/// let auth = openstack::auth::Identity::from_env()
///     .expect("Unable to authenticate");
/// let session = openstack::Session::new(auth);
/// let server_list = openstack::compute::v2::servers(&session).list()
///     .expect("Unable to fetch servers");
/// ```
///
/// Sorting servers by `access_ip_v4` and getting first 5 results:
///
/// ```rust,no_run
/// use openstack;
///
/// let auth = openstack::auth::Identity::from_env()
///     .expect("Unable to authenticate");
/// let session = openstack::Session::new(auth);
/// let sorting = openstack::compute::v2::ServerSortKey::AccessIpv4;
/// let server_list = openstack::compute::v2::servers(&session).query()
///     .sort_by(openstack::Sort::Asc(sorting)).with_limit(5)
///     .fetch().expect("Unable to fetch servers");
/// ```
///
/// Fetching server details by its UUID:
///
/// ```rust,no_run
/// use openstack;
///
/// let auth = openstack::auth::Identity::from_env()
///     .expect("Unable to authenticate");
/// let session = openstack::Session::new(auth);
/// let server = openstack::compute::v2::servers(&session)
///     .get("8a1c355b-2e1e-440a-8aa8-f272df72bc32")
///     .expect("Unable to get a server");
/// println!("Server name is {}, image ID is {}, flavor ID is {}",
///          server.name(), server.image().id(), server.flavor().id());
/// ```
#[derive(Clone, Debug)]
pub struct ServerManager<'s> {
    service: &'s ComputeV2
}

/// Structure representing a summary of a single server.
#[derive(Clone, Debug)]
pub struct Server<'s> {
    service: &'s ComputeV2,
    inner: protocol::Server
}

/// Structure representing a summary of a single server.
#[derive(Clone, Debug)]
pub struct ServerSummary<'s> {
    service: &'s ComputeV2,
    inner: protocol::ServerSummary
}

/// List of servers.
pub type ServerList<'s> = Vec<ServerSummary<'s>>;

/// A reference to a flavor.
#[derive(Clone, Copy, Debug)]
pub struct FlavorRef<'s> {
    server: &'s Server<'s>
}

/// A reference to an image.
#[derive(Clone, Copy, Debug)]
pub struct ImageRef<'s> {
    server: &'s Server<'s>
}


impl<'s> Server<'s> {
    /// Get a reference to IPv4 address.
    pub fn access_ipv4(&self) -> &Option<Ipv4Addr> {
        &self.inner.accessIPv4
    }

    /// Get a reference to IPv6 address.
    pub fn access_ipv6(&self) -> &Option<Ipv6Addr> {
        &self.inner.accessIPv6
    }

    /// Get a reference to associated addresses.
    pub fn addresses(&self) -> &HashMap<String, Vec<protocol::ServerAddress>> {
        &self.inner.addresses
    }

    /// Get a reference to the availability zone.
    pub fn availability_zone(&self) -> &String {
        &self.inner.availability_zone
    }

    /// Get a reference to creation date and time.
    pub fn created_at(&self) -> &DateTime<FixedOffset> {
        &self.inner.created
    }

    /// Get a reference to the flavor.
    pub fn flavor(&'s self) -> FlavorRef<'s> {
        FlavorRef {
            server: self
        }
    }

    /// Get a reference to server unique ID.
    pub fn id(&self) -> &String {
        &self.inner.id
    }

    /// Get a reference to the image.
    pub fn image(&'s self) -> ImageRef<'s> {
        ImageRef {
            server: self
        }
    }

    /// Get a reference to server name.
    pub fn name(&self) -> &String {
        &self.inner.name
    }

    /// Get server status.
    pub fn status(&self) -> protocol::ServerStatus {
        self.inner.status
    }

    /// Get a reference to last update date and time.
    pub fn updated_at(&self) -> &DateTime<FixedOffset> {
        &self.inner.updated
    }
}

impl<'s> FlavorRef<'s> {
    /// Get a reference to flavor unique ID.
    pub fn id(&self) -> &'s String {
        &self.server.inner.flavor.id
    }

    // TODO: pub fn details(&self) -> ApiResult<Flavor>
}

impl<'s> ImageRef<'s> {
    /// Get a reference to image unique ID.
    pub fn id(&self) -> &'s String {
        &self.server.inner.image.id
    }

    // TODO: #[cfg(feature = "image")] pub fn details(&self) -> ApiResult<Image>
}

impl<'s> ServerSummary<'s> {
    /// Get a reference to server unique ID.
    pub fn id(&self) -> &String {
        &self.inner.id
    }

    /// Get a reference to server name.
    pub fn name(&self) -> &String {
        &self.inner.name
    }

    /// Get details.
    pub fn details(&self) -> ApiResult<Server<'s>> {
        ServerManager::get_server(self.service.clone(), &self.inner.id)
    }
}

impl<'s> ServerQuery<'s> {
    fn new(service: &'s ComputeV2) -> ServerQuery<'s> {
        ServerQuery {
            service: service,
            query: Query::new(),
        }
    }

    /// Add marker to the request.
    pub fn with_marker<T: Into<String>>(mut self, marker: T) -> Self {
        self.query.push_str("marker", marker);
        self
    }

    /// Add limit to the request.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.query.push("limit", limit);
        self
    }

    /// Add sorting to the request.
    pub fn sort_by(mut self, sort: Sort<protocol::ServerSortKey>) -> Self {
        let (field, direction) = sort.into();
        self.query.push_str("sort_key", field);
        self.query.push("sort_dir", direction);
        self
    }

    /// Filter by IPv4 address that should be used to access the server.
    pub fn with_access_ip_v4(mut self, value: Ipv4Addr) -> Self {
        self.query.push("access_ip_v4", value);
        self
    }

    /// Filter by IPv6 address that should be used to access the server.
    pub fn with_access_ip_v6(mut self, value: Ipv6Addr) -> Self {
        self.query.push("access_ipv6", value);
        self
    }

    /// Filter by availability zone.
    pub fn with_availability_zone<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("availability_zone", value);
        self
    }

    /// Filter by flavor.
    pub fn with_flavor<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("flavor", value);
        self
    }

    /// Filter by host name.
    pub fn with_hostname<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("hostname", value);
        self
    }

    /// Filter by image ID.
    pub fn with_image<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("image", value);
        self
    }

    /// Filter by an IPv4 address.
    pub fn with_ip_v4(mut self, value: Ipv4Addr) -> Self {
        self.query.push("ip", value);
        self
    }

    /// Filter by an IPv6 address.
    pub fn with_ip_v6(mut self, value: Ipv6Addr) -> Self {
        self.query.push("ip6", value);
        self
    }

    /// Filter by server name (a database regular expression).
    pub fn with_name<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("name", value);
        self
    }

    /// Filter by power state.
    pub fn with_power_state<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("power_state", value);
        self
    }

    /// Filter by project ID (also commonly known as tenant ID).
    pub fn with_project_id<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("project_id", value);
        self
    }

    /// Filter by server status.
    pub fn with_status<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("status", value);
        self
    }

    /// Filter by user ID.
    pub fn with_user_id<T: Into<String>>(mut self, value: T) -> Self {
        self.query.push_str("user_id", value);
        self
    }

    /// Execute this request and return its result.
    #[allow(unused_results)]
    pub fn fetch(self) -> ApiResult<ServerList<'s>> {
        let service = self.service;
        let query = self.query;

        trace!("Listing compute servers with {:?}", query);
        let inner: protocol::ServersRoot = service.get_json(&["servers"],
                                                            query)?;
        debug!("Received {} compute servers", inner.servers.len());
        trace!("Received servers: {:?}", inner.servers);
        Ok(inner.servers.into_iter().map(|x| ServerSummary {
            service: service.clone(),
            inner: x
        }).collect())
    }
}

impl<'s> ServerManager<'s> {
    /// Constructor for server manager.
    pub fn new(service: &'s ComputeV2) -> ServerManager<'s> {
        ServerManager {
            service: service
        }
    }

    /// Run a query against server list.
    ///
    /// Note that this method does not return results immediately, but rather
    /// a [ServerQuery](struct.ServerQuery.html) object that
    /// you can futher specify with e.g. filtering or sorting.
    pub fn query(&self) -> ServerQuery<'s> {
        ServerQuery::new(self.service)
    }

    /// List all servers.
    pub fn list(&self) -> ApiResult<ServerList<'s>> {
        self.query().fetch()
    }

    /// Get a server.
    pub fn get<Id: AsRef<str>>(&self, id: Id) -> ApiResult<Server<'s>> {
        ServerManager::get_server(self.service.clone(), id.as_ref())
    }

    fn get_server(service: &'s ComputeV2, id: &str) -> ApiResult<Server<'s>> {
        trace!("Get compute server {}", id);
        let inner: protocol::ServerRoot = service.get_json(&["servers", id],
                                                           Query::new())?;
        trace!("Received {:?}", inner.server);
        Ok(Server {
            service: service,
            inner: inner.server
        })
    }
}


#[cfg(test)]
pub mod test {
    #![allow(missing_debug_implementations)]
    #![allow(unused_results)]

    use hyper;

    use super::super::super::super::auth::NoAuth;
    use super::super::super::super::session::test;
    use super::super::base::test as api_test;
    use super::ServerManager;

    const SERVERS_RESPONSE: &'static str = r#"
    {
        "servers": [
            {
                "id": "22c91117-08de-4894-9aa9-6ef382400985",
                "links": [
                    {
                        "href": "http://openstack.example.com/v2/6f70656e737461636b20342065766572/servers/22c91117-08de-4894-9aa9-6ef382400985",
                        "rel": "self"
                    },
                    {
                        "href": "http://openstack.example.com/6f70656e737461636b20342065766572/servers/22c91117-08de-4894-9aa9-6ef382400985",
                        "rel": "bookmark"
                    }
                ],
                "name": "new-server-test"
            }
        ]
    }"#;

    mock_connector_in_order!(MockServers {
        String::from("HTTP/1.1 200 OK\r\nServer: Mock.Mock\r\n\
                     \r\n") + api_test::ONE_VERSION_RESPONSE
        String::from("HTTP/1.1 200 OK\r\nServer: Mock.Mock\r\n\
                     \r\n") + SERVERS_RESPONSE
    });

    #[test]
    fn test_servers_list() {
        let auth = NoAuth::new("http://127.0.2.1/v2.1").unwrap();
        let cli = hyper::Client::with_connector(MockServers::default());
        let session = test::new_with_params(auth, cli, None);

        let mgr = ServerManager::new(&session);
        let srvs = mgr.list().unwrap();
        assert_eq!(srvs.len(), 1);
        assert_eq!(srvs[0].id(),
                   "22c91117-08de-4894-9aa9-6ef382400985");
        assert_eq!(srvs[0].name(), "new-server-test");
    }
}
