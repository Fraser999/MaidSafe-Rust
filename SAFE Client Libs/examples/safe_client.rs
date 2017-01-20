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

//! Safe client example.

// For explanation of lint checks, run `rustc -W help` or see
// https://github.com/maidsafe/QA/blob/master/Documentation/Rust%20Lint%20Checks.md
#![forbid(bad_style, exceeding_bitshifts, mutable_transmutes, no_mangle_const_items,
          unknown_crate_types, warnings)]
#![deny(deprecated, improper_ctypes, missing_docs,
        non_shorthand_field_patterns, overflowing_literals, plugin_as_library,
        private_no_mangle_fns, private_no_mangle_statics, stable_features, unconditional_recursion,
        unknown_lints, unsafe_code, unused, unused_allocation, unused_attributes,
        unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results)]
#![allow(box_pointers, fat_ptr_transmutes, missing_copy_implementations,
         missing_debug_implementations, variant_size_differences)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(feature="clippy", deny(clippy, clippy_pedantic))]
#![cfg_attr(feature="clippy", allow(use_debug, print_stdout))]

extern crate routing;
extern crate safe_core;
extern crate rust_sodium;
#[macro_use]
extern crate maidsafe_utilities;
#[macro_use]
extern crate unwrap;

use routing::messaging::MpidMessageWrapper;
use rust_sodium::crypto::hash::sha256;
use safe_core::core::client::Client;
use safe_core::core::client::response_getter::GetResponseGetter;

#[cfg(feature = "use-mock-routing")]
const MOCK_NETWORK: bool = true;
#[cfg(not(feature = "use-mock-routing"))]
const MOCK_NETWORK: bool = false;

fn main() {
    unwrap!(maidsafe_utilities::log::init(true));

    let mut secret_0 = String::new();
    let mut secret_1 = String::new();

    println!("\nDo you already have an account created (enter Y for yes) ?");

    let mut user_option = String::new();
    let _ = std::io::stdin().read_line(&mut user_option);
    user_option = user_option.trim().to_string();

    if user_option != "Y" && user_option != "y" {
        println!("\n\tAccount Creation");
        println!("\t================");

        println!("\n------------ Enter account-locator ---------------");
        let _ = std::io::stdin().read_line(&mut secret_0);
        secret_0 = secret_0.trim().to_string();
        println!("\n------------ Enter password ---------------");
        let _ = std::io::stdin().read_line(&mut secret_1);
        secret_1 = secret_1.trim().to_string();

        // Account Creation
        {
            println!("\nTrying to create an account ...");

            let _ = unwrap!(Client::create_account(&secret_0, &secret_1));
            println!("Account Created Successfully !!");
        }

        println!("\n\n\tAuto Account Login");
        println!("\t==================");

        // Log into the created account
        {
            println!("\nTrying to log into the created account using supplied credentials ...");

            let _ = unwrap!(Client::log_in(&secret_0, &secret_1));
            println!("Account Login Successful !!");
        }
    }

    println!("\n\n\tManual Account Login");
    println!("\t====================");

    loop {
        secret_0 = String::new();
        secret_1 = String::new();

        println!("\n------------ Enter account-locator ---------------");
        let _ = std::io::stdin().read_line(&mut secret_0);
        secret_0 = secret_0.trim().to_string();

        println!("\n------------ Enter password ---------------");
        let _ = std::io::stdin().read_line(&mut secret_1);
        secret_1 = secret_1.trim().to_string();

        // Log into the created account
        {
            println!("\nTrying to log in ...");
            match Client::log_in(&secret_0, &secret_1) {
                Ok(mut client) => {
                    println!("Account Login Successful !!");
                    if MOCK_NETWORK {
                        println!("Messaging feature has been skipped as mock routing is being \
                                  used !!");
                    } else {
                        // messaging(&mut client);
                    }
                    break;
                }
                Err(error) => println!("Account Login Failed !! Reason: {:?}\n\n", error),
            }
        }
    }
}

// fn messaging(client: &mut Client) {
//     println!("\nDo you want to continue with the mpid messaging feature (enter Y for yes) ?");
//     let mut messaging_option = String::new();
//     let _ = std::io::stdin().read_line(&mut messaging_option);
//     messaging_option = messaging_option.trim().to_string();
//     if messaging_option != "Y" && messaging_option != "y" {
//         return;
//     }

//     println!("\n------------ Creating mpid account, enter a memorable name ---------------");
//     let mut account_name = String::new();
//     let _ = std::io::stdin().read_line(&mut account_name);
//     let mpid_account = XorName(sha256::hash(&account_name.into_bytes()).0);
//     let response_getter = unwrap!(client.register_online(mpid_account));

//     loop {
//         println!("\n------- messaging options: r for receive, s for send, t for terminate -------");
//         let mut operation = String::new();
//         let _ = std::io::stdin().read_line(&mut operation);
//         operation = operation.trim().to_string();
//         if operation == "r" {
//             receive_mpid_message(&response_getter);
//         } else if operation == "s" {
//             send_mpid_message(client, mpid_account);
//         } else if operation == "t" {
//             break;
//         }
//     }
// }

// fn receive_mpid_message(response_getter: &GetResponseGetter) {
//     loop {
//         if let Ok(data) = response_getter.get() {
//             match data {
//                 Data::Plain(plain_data) => {
//                     let mpid_message_wrapper: MpidMessageWrapper =
//                         unwrap!(deserialise(plain_data.value()));
//                     match mpid_message_wrapper {
//                         MpidMessageWrapper::PutMessage(mpid_message) => {
//                             println!("received mpid message {:?}", mpid_message);
//                             break;
//                         }
//                         _ => println!("unknown received mpid_message_wrapper"),
//                     }
//                 }
//                 _ => println!("unknown received data"),
//             }
//         }
//         std::thread::sleep(std::time::Duration::from_millis(1000));
//     }
// }

// fn send_mpid_message(client: &mut Client, mpid_account: XorName) {
//     let mut receiver_name = String::new();
//     let mut msg_metadata = String::new();
//     let mut msg_content = String::new();
//     println!("\n------------ enter receiver's memorable name ---------------");
//     let _ = std::io::stdin().read_line(&mut receiver_name);
//     let receiver_account = XorName(sha256::hash(&receiver_name.into_bytes()).0);
//     println!("\n------------ enter metadata of the message ---------------");
//     let _ = std::io::stdin().read_line(&mut msg_metadata);
//     println!("\n------------ enter content of the message ---------------");
//     let _ = std::io::stdin().read_line(&mut msg_content);
//     let secret_key = unwrap!(client.get_secret_signing_key()).clone();
//     let _ = client.send_message(mpid_account,
//                                 msg_metadata.into_bytes(),
//                                 msg_content.into_bytes(),
//                                 receiver_account,
//                                 &secret_key);
// }
