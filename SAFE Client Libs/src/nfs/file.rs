// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.


use nfs::errors::NfsError;
use nfs::metadata::file_metadata::FileMetadata;
use rand::{OsRng, Rand};
use routing::XorName;
use self_encryption::DataMap;
use std::fmt;

/// Representation of a File to be put into the network. Could be text, music, video etc any kind
/// of file
#[derive(RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct File {
    id: XorName,
    metadata: FileMetadata,
    datamap: DataMap,
}

impl File {
    /// Create a new instance of File
    pub fn new(metadata: FileMetadata, datamap: DataMap) -> Result<File, NfsError> {
        Ok(File {
            id: Rand::rand(&mut unwrap!(OsRng::new(), "Failed to create OsRng.")),
            metadata: metadata,
            datamap: datamap,
        })
    }

    /// Returns unique id
    pub fn get_id(&self) -> &XorName {
        &self.id
    }

    /// Get the name of the File
    pub fn get_name(&self) -> &str {
        self.metadata.get_name()
    }

    /// Get metadata associated with the file
    pub fn get_metadata(&self) -> &FileMetadata {
        &self.metadata
    }

    /// Get metadata associated with the file, with mutability to allow updation
    pub fn get_mut_metadata(&mut self) -> &mut FileMetadata {
        &mut self.metadata
    }

    /// Get the data-map of the File. This is generated by passing the contents of the File to
    /// self-encryption
    pub fn get_datamap(&self) -> &DataMap {
        &self.datamap
    }

    /// Set a data-map to be associated with the File
    pub fn set_datamap(&mut self, datamap: DataMap) {
        self.datamap = datamap;
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File > metadata: {:?}", self.metadata)
    }
}

#[cfg(test)]
mod test {
    use maidsafe_utilities::serialisation::{deserialise, serialise};
    use nfs::metadata::file_metadata::FileMetadata;
    use self_encryption::DataMap;
    use super::*;

    #[test]
    fn serialise_deserialise() {
        let obj_before = unwrap!(File::new(FileMetadata::new("Home".to_string(),
                                                             "{mime:\"application/json\"}"
                                                                 .to_string()
                                                                 .into_bytes()),
                                           DataMap::None));
        let serialised_data = unwrap!(serialise(&obj_before));
        let obj_after = unwrap!(deserialise(&serialised_data));
        assert_eq!(obj_before, obj_after);
    }
}
