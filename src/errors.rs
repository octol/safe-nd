// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use crate::ADataEntries;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error,
    fmt::{self, Debug, Display, Formatter},
    result,
};

/// A specialised `Result` type for safecoin.
pub type Result<T> = result::Result<T, Error>;

/// Error debug struct
pub struct ErrorDebug<'a, T>(pub &'a Result<T>);

impl<'a, T> Debug for ErrorDebug<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Err(error) = self.0 {
            write!(f, "{:?}", error)
        } else {
            write!(f, "Success")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Error {
    /// Access is denied for a given requester
    AccessDenied,
    /// Login packet does not exist
    NoSuchLoginPacket,
    /// Attempt to store a login packet at an already occupied address
    LoginPacketExists,
    /// Requested data not found
    NoSuchData,
    /// Attempt to create a mutable data when data with such a name already exists
    DataExists,
    /// Requested entry not found
    NoSuchEntry,
    /// Exceeded a limit on a number of entries
    TooManyEntries,
    /// Some entry actions are not valid.
    InvalidEntryActions(BTreeMap<Vec<u8>, EntryError>),
    /// Key does not exist
    NoSuchKey,
    /// The key(s) of the entry or entries contained in this error already exist
    KeysExist(ADataEntries),
    /// Duplicate Entries in this push
    DuplicateEntryKeys,
    /// The list of owner keys is invalid
    InvalidOwners,
    /// Invalid version for performing a given mutating operation. Contains the
    /// current data version.
    InvalidSuccessor(u64),
    /// Invalid version for performing a given mutating operation. Contains the
    /// current owners version.
    InvalidOwnersSuccessor(u64),
    /// Invalid version for performing a given mutating operation. Contains the
    /// current permissions version.
    InvalidPermissionsSuccessor(u64),
    /// The list of permissions is invalid.
    InvalidPermissions,
    /// Invalid Operation such as a POST on ImmutableData
    InvalidOperation,
    /// Mismatch between key type and signature type.
    SigningKeyTypeMismatch,
    /// Failed signature validation.
    InvalidSignature,
    /// Received a request with a duplicate MessageId
    DuplicateMessageId,
    /// Network error occurring at Vault level which has no bearing on clients, e.g. serialisation
    /// failure or database failure
    NetworkOther(String),
    /// While parsing, precision would be lost.
    LossOfPrecision,
    /// The coin amount would exceed
    /// [the maximum value for `Coins`](constant.MAX_COINS_VALUE.html).
    ExcessiveValue,
    /// Failed to parse the string as [`Coins`](struct.Coins.html).
    FailedToParse(String),
    /// Transaction ID already exists.
    TransactionIdExists,
    /// Insufficient coins.
    InsufficientBalance,
    /// Inexistent balance.
    NoSuchBalance,
    /// Coin balance already exists.
    BalanceExists,
    /// Expected data size exceeded.
    ExceededSize,
}

impl<T: Into<String>> From<T> for Error {
    fn from(err: T) -> Self {
        Error::NetworkOther(err.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Error::AccessDenied => write!(f, "Access denied"),
            Error::NoSuchLoginPacket => write!(f, "Login packet does not exist"),
            Error::LoginPacketExists => write!(f, "Login packet already exists at this location"),
            Error::NoSuchData => write!(f, "Requested data not found"),
            Error::DataExists => write!(f, "Data given already exists"),
            Error::NoSuchEntry => write!(f, "Requested entry not found"),
            Error::TooManyEntries => write!(f, "Exceeded a limit on a number of entries"),
            Error::InvalidEntryActions(ref errors) => {
                write!(f, "Entry actions are invalid: {:?}", errors)
            }
            Error::NoSuchKey => write!(f, "Key does not exists"),
            Error::KeysExist(_) => write!(f, "Key(s) already exists"),
            Error::DuplicateEntryKeys => write!(f, "Duplicate keys in this push"),
            Error::InvalidOwners => write!(f, "The list of owner keys is invalid"),
            Error::InvalidOperation => write!(f, "Requested operation is not allowed"),
            Error::InvalidSuccessor(_) => {
                write!(f, "Data given is not a valid successor of stored data")
            }
            Error::InvalidOwnersSuccessor(_) => {
                // TODO
                write!(f, "Data given is not a valid successor of stored data")
            }
            Error::InvalidPermissionsSuccessor(_) => {
                // TODO
                write!(f, "Data given is not a valid successor of stored data")
            }
            Error::InvalidPermissions => write!(f, "The lists of permissions is invalid"),
            Error::SigningKeyTypeMismatch => {
                write!(f, "Mismatch between key type and signature type")
            }
            Error::InvalidSignature => write!(f, "Failed signature validation"),
            Error::NetworkOther(ref error) => write!(f, "Error on Vault network: {}", error),
            Error::LossOfPrecision => {
                write!(f, "Lost precision on the number of coins during parsing")
            }
            Error::ExcessiveValue => write!(
                f,
                "Overflow on number of coins (check the MAX_COINS_VALUE const)"
            ),
            Error::FailedToParse(ref error) => {
                write!(f, "Failed to parse from a string: {}", error)
            }
            Error::TransactionIdExists => write!(f, "Transaction with a given ID already exists"),
            Error::InsufficientBalance => write!(f, "Not enough coins to complete this operation"),
            Error::NoSuchBalance => write!(f, "Balance does not exist"),
            Error::BalanceExists => write!(f, "Balance already exists"),
            Error::DuplicateMessageId => write!(f, "MessageId already exists"),
            Error::ExceededSize => write!(f, "Size of the structure exceeds the limit"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AccessDenied => "Access denied",
            Error::NoSuchLoginPacket => "Login packet does not exist",
            Error::LoginPacketExists => "Login packet already exists at this location",
            Error::NoSuchData => "No such data",
            Error::DataExists => "Data exists",
            Error::NoSuchEntry => "No such entry",
            Error::TooManyEntries => "Too many entries",
            Error::InvalidEntryActions(_) => "Invalid entry actions",
            Error::NoSuchKey => "No such key",
            Error::KeysExist(_) => "Key(s) already exist",
            Error::DuplicateEntryKeys => "Duplicate keys in this push",
            Error::InvalidOwners => "Invalid owners",
            Error::InvalidSuccessor(_) => "Invalid data successor",
            Error::InvalidOwnersSuccessor(_) => "Invalid owners successor",
            Error::InvalidPermissionsSuccessor(_) => "Invalid permissions successor",
            Error::InvalidPermissions => "Invalid permissions",
            Error::InvalidOperation => "Invalid operation",
            Error::SigningKeyTypeMismatch => "Key type and signature type mismatch",
            Error::InvalidSignature => "Invalid signature",
            Error::NetworkOther(ref error) => error,
            Error::LossOfPrecision => "Lost precision on the number of coins during parsing",
            Error::ExcessiveValue => {
                "Overflow on number of coins (check the MAX_COINS_VALUE const)"
            }
            Error::FailedToParse(_) => "Failed to parse entity",
            Error::TransactionIdExists => "Transaction with a given ID already exists",
            Error::InsufficientBalance => "Not enough coins to complete this operation",
            Error::NoSuchBalance => "Balance does not exist",
            Error::BalanceExists => "Balance already exists",
            Error::DuplicateMessageId => "MessageId already exists",
            Error::ExceededSize => "Exceeded the size limit",
        }
    }
}

/// Entry error for `Error::InvalidEntryActions`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntryError {
    /// Entry does not exists.
    NoSuchEntry,
    /// Entry already exists. Contains the current entry Key.
    EntryExists(u8),
    /// Invalid version when updating an entry. Contains the current entry Key.
    InvalidSuccessor(u8),
}
