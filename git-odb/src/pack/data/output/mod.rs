mod count {
    use crate::{compound::find::PackLocation, data, pack::data::output::entry};
    use git_hash::ObjectId;

    /// An item representing a future Entry in the leanest way possible.
    ///
    /// One can expect to have one of these in memory when building big objects, so smaller is better here.
    /// They should contain everything of importance to generate a pack as fast as possible.
    #[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
    #[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
    pub struct Count {
        /// The hash of the object to write
        pub id: ObjectId,
        /// The kind of packed object
        pub object_kind: git_object::Kind,
        /// The size in bytes needed once `data` gets decompressed
        pub decompressed_size: usize,

        /// The kind of entry represented by `data`. It's used alongside with it to complete the pack entry
        /// at rest or in transit.
        pub entry_kind: entry::Kind,
        /// A way to locate a pack entry in the object database, only available if the object is in a pack.
        pub(crate) entry_pack_location: Option<PackLocation>,
    }

    /// The error returned by [`output::Entry::from_data()`].
    #[allow(missing_docs)]
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("{0}")]
        ZlibDeflate(#[from] std::io::Error),
    }

    impl Count {
        /// Create a new instance from the given `oid` and its corresponding git `obj`ect data.
        pub fn from_data(oid: impl Into<ObjectId>, obj: &data::Object<'_>) -> Result<Self, Error> {
            Ok(Count {
                id: oid.into(),
                object_kind: obj.kind,
                entry_kind: entry::Kind::Base,
                decompressed_size: obj.data.len(),
                entry_pack_location: None, // TODO: actual pack location
            })
        }
    }
}
#[doc(inline)]
pub use count::Count;

///
pub mod entry;
#[doc(inline)]
pub use entry::Entry;

///
pub mod objects_to_entries;
pub use objects_to_entries::objects_to_entries_iter;

///
pub mod count_objects;
pub use count_objects::count_objects_iter;

///
pub mod entries_to_bytes;
pub use entries_to_bytes::EntriesToBytesIter;
