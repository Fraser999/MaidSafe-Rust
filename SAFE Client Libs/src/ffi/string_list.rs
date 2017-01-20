// Copyright 2016 MaidSafe.net limited.
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

//! StringList is a FFI-enabled wrapper over vector of strings with a corresponding
//! FFI-enabled API.


use ffi::errors::FfiError;
use libc::c_char;
use std::ffi::CString;
use std::ptr;

/// List of strings.
pub type StringList = Vec<CString>;

/// Convert vec of strings to an owning raw pointer to StringList. Consumes the
/// vector.
pub fn into_ptr(strings: Vec<String>) -> Result<*mut StringList, FfiError> {
    let mut result = Vec::with_capacity(strings.len());

    for string in strings {
        result.push(try!(CString::new(string)));
    }

    Ok(Box::into_raw(Box::new(result)))
}

/// Get number of elements in the string list.
#[no_mangle]
pub unsafe extern "C" fn string_list_len(list: *const StringList) -> usize {
    (*list).len()
}

/// Get the string at the given index, or NULL if the index is out of range.
#[no_mangle]
pub unsafe extern "C" fn string_list_at(list: *const StringList, index: usize) -> *const c_char {
    let list = &*list;

    if index < list.len() {
        list[index].as_ptr()
    } else {
        ptr::null()
    }
}

/// Dispose of the string list.
#[no_mangle]
pub unsafe extern "C" fn string_list_drop(list: *mut StringList) {
    let _ = Box::from_raw(list);
}

#[cfg(test)]
mod test {
    use std::ffi::CStr;
    use std::ptr;

    #[test]
    fn smoke() {
        let strings = vec!["one".to_string(), "two".to_string()];

        unsafe {
            let list = unwrap!(super::into_ptr(strings));
            assert_eq!(super::string_list_len(list), 2);

            let item = unwrap!(CStr::from_ptr(super::string_list_at(list, 0)).to_str());
            assert_eq!(item, "one");
            assert_eq!(super::string_list_at(list, 2), ptr::null());

            super::string_list_drop(list);
        }
    }
}
