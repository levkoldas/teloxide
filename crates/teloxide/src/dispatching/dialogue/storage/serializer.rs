//! Various serializers for dialogue storages.

use serde::{de::DeserializeOwned, ser::Serialize};

/// A serializer for memory storages.
pub trait Serializer<D> {
    type Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error>;
    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error>;
}

/// The JSON serializer for memory storages.
pub struct Json;

impl<D> Serializer<D> for Json
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_json::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_json::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_json::from_slice(data)
    }
}

/// The [CBOR] serializer for memory storages.
///
/// [CBOR]: https://en.wikipedia.org/wiki/CBOR
#[cfg(feature = "cbor-serializer")]
pub struct Cbor;

#[cfg(feature = "cbor-serializer")]
impl<D> Serializer<D> for Cbor
where
    D: Serialize + DeserializeOwned,
{
    type Error = serde_cbor::Error;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        serde_cbor::to_vec(val)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        serde_cbor::from_slice(data)
    }
}

/// The [Bincode] serializer for memory storages.
///
/// Can't serialize if the length is unknown, [see this issue](https://github.com/bincode-org/bincode/issues/167).
/// Use [`Cbor`] or [`Json`] if you are storing structs like [`Message`].
///
/// Uses the legacy fixed-width encoding, compatible with dialogue data written
/// by bincode 1.x. Trailing bytes are accepted, as in the previous serializer.
///
/// [Bincode]: https://docs.rs/bincode/2.0.1/bincode/
/// [`Cbor`]: crate::dispatching::dialogue::serializer::Cbor
/// [`Json`]: crate::dispatching::dialogue::serializer::Json
/// [`Message`]: crate::types::Message
#[cfg(feature = "bincode-serializer")]
pub struct Bincode;

#[cfg(feature = "bincode-serializer")]
impl<D> Serializer<D> for Bincode
where
    D: Serialize + DeserializeOwned,
{
    type Error = BincodeError;

    fn serialize(&self, val: &D) -> Result<Vec<u8>, Self::Error> {
        bincode::serde::encode_to_vec(val, bincode::config::legacy()).map_err(Into::into)
    }

    fn deserialize(&self, data: &[u8]) -> Result<D, Self::Error> {
        bincode::serde::decode_from_slice(data, bincode::config::legacy())
            .map(|(value, _)| value)
            .map_err(Into::into)
    }
}

/// An error encoding or decoding a bincode dialogue.
#[cfg(feature = "bincode-serializer")]
#[derive(Debug, thiserror::Error)]
pub enum BincodeError {
    #[error(transparent)]
    Encode(#[from] bincode::error::EncodeError),
    #[error(transparent)]
    Decode(#[from] bincode::error::DecodeError),
}

#[cfg(all(test, feature = "bincode-serializer"))]
mod tests {
    use super::{Bincode, Serializer};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum State {
        Start,
        Name { name: String, age: u16 },
    }

    #[test]
    fn bincode_preserves_existing_dialogue_encoding() {
        // bincode 1.x's free functions encode enum indices as u32, lengths as
        // u64 and integers in little endian, without variable-length encoding.
        let legacy = [1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, b'A', b'd', b'a', 37, 0];
        let state = State::Name { name: "Ada".into(), age: 37 };
        assert_eq!(Bincode.serialize(&state).unwrap(), legacy);
        assert_eq!(<Bincode as Serializer<State>>::deserialize(&Bincode, &legacy).unwrap(), state);
        let mut trailing = legacy.to_vec();
        trailing.push(0xff);
        assert_eq!(
            <Bincode as Serializer<State>>::deserialize(&Bincode, &trailing).unwrap(),
            state
        );
        assert!(<Bincode as Serializer<State>>::deserialize(&Bincode, &legacy[..5]).is_err());
    }
}
