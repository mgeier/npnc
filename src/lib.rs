// Copyright 2017 Kyle Mayes
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

//! Lock-free queues.

#![warn(missing_copy_implementations, missing_debug_implementations, missing_docs)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", warn(clippy))]

use std::error;
use std::fmt;

//================================================
// Enums
//================================================

// ConsumeError __________________________________

/// Indicates the reason a `consume` operation could not return an item.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConsumeError {
    /// The queue was empty and had no remaining producers.
    Disconnected,
    /// The queue was empty.
    Empty,
}

impl error::Error for ConsumeError {
    fn description(&self) -> &str {
        match *self {
            ConsumeError::Disconnected => "the queue was empty and had no remaining producers",
            ConsumeError::Empty => "the queue was empty",
        }
    }
}

impl fmt::Display for ConsumeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", error::Error::description(self))
    }
}
