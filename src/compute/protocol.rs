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

//! JSON structures and protocol bits for the Compute API.

#![allow(non_snake_case)]
#![allow(missing_docs)]

use super::super::session::ServiceType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub accessIPv4: String,
    pub accessIPv6: String,
    pub id: String,
    pub name: String,
    pub status: String,
    pub tenant_id: String,
    pub user_id: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSummary {
    pub id: String,
    pub name: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServersRoot {
    pub servers: Vec<ServerSummary>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerRoot {
    pub server: Server
}

/// Marker for Compute API.
#[derive(Copy, Clone, Debug)]
pub struct ComputeApiV2;

const SERVICE_TYPE: &'static str = "compute";
const SUFFIX: &'static str = "v2.1";

impl ServiceType for ComputeApiV2 {
    fn catalog_type() -> &'static str {
        SERVICE_TYPE
    }

    fn version_suffix() -> Option<&'static str> {
        Some(SUFFIX)
    }
}
