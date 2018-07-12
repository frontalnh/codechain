// Copyright 2018 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::result::Result;

use primitives::H256;

use super::addr::SocketAddr;

pub trait Control: Send + Sync {
    fn register_secret(&self, secret: H256, addr: SocketAddr);
    fn connect(&self, addr: SocketAddr);
    fn disconnect(&self, addr: SocketAddr) -> Result<(), Error>;
    fn is_connected(&self, addr: &SocketAddr) -> bool;
}

#[derive(Clone, Debug)]
pub enum Error {
    NotConnected,
}
