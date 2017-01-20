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


use core::client::message_queue::MessageQueue;
use core::errors::CoreError;
use core::translated_events::ResponseEvent;
use maidsafe_utilities;
use routing::{Data, DataIdentifier, XorName};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

const REQ_TIMEOUT_SECS: u64 = 120;

// TODO - consider using template specialisation (if it becomes available) for these three structs
//        which all do similar things.

/// `GetResponseGetter` is a lazy evaluated response getter for GET Requests. It will fetch either
/// from local cache or wait for the `MessageQueue` to notify it of the incoming response from the
/// network.
pub struct GetResponseGetter {
    data_channel: Option<(Sender<ResponseEvent>, Receiver<ResponseEvent>)>,
    message_queue: Arc<Mutex<MessageQueue>>,
    requested_name: XorName,
    requested_id: DataIdentifier,
}

impl GetResponseGetter {
    /// Create a new instance of `GetResponseGetter`
    pub fn new(data_channel: Option<(Sender<ResponseEvent>, Receiver<ResponseEvent>)>,
               message_queue: Arc<Mutex<MessageQueue>>,
               requested_id: DataIdentifier)
               -> GetResponseGetter {
        GetResponseGetter {
            data_channel: data_channel,
            message_queue: message_queue,
            requested_name: *requested_id.name(),
            requested_id: requested_id,
        }
    }

    /// Get either from local cache or (if not available there) get it when it comes from the
    /// network as informed by MessageQueue. This is blocking.
    pub fn get(&self) -> Result<Data, CoreError> {
        if let Some((ref sender, ref data_receiver)) = self.data_channel {
            let sender = sender.clone();
            let (tx, rx) = mpsc::channel();
            let _thread_canceller = ThreadCanceller(tx);
            wait_canceller(sender, rx);
            match try!(data_receiver.recv()) {
                ResponseEvent::GetResp(result) => {
                    let data = try!(result);
                    if let DataIdentifier::Immutable(..) = self.requested_id {
                        let mut msg_queue = unwrap!(self.message_queue.lock());
                        msg_queue.local_cache_insert(self.requested_name, data.clone());
                    }

                    Ok(data)
                }
                ResponseEvent::Terminated => Err(CoreError::RequestTimeout),
                _ => Err(CoreError::ReceivedUnexpectedData),
            }
        } else {
            let mut msg_queue = unwrap!(self.message_queue.lock());
            msg_queue.local_cache_get(&self.requested_name)
        }
    }

    /// Extract associated sender. This will help cancel the blocking wait at will if so desired.
    /// All that is needed is to extract the sender before doing a `get()` and then while blocking
    /// on `get()` fire `sender.send(ResponseEvent::Terminated)` to gracefully exit the receiver.
    pub fn get_sender(&self) -> Option<&Sender<ResponseEvent>> {
        self.data_channel.as_ref().and_then(|&(ref sender, _)| Some(sender))
    }
}

/// `GetAccountInfoResponseGetter` is a lazy evaluated response getter for `GetAccountInfo`
/// Requests. It will wait for the `MessageQueue` to notify it of the incoming response from the
/// network.
pub struct GetAccountInfoResponseGetter {
    data_channel: (Sender<ResponseEvent>, Receiver<ResponseEvent>),
}

impl GetAccountInfoResponseGetter {
    /// Create a new instance of `GetAccountInfoResponseGetter`
    pub fn new(data_channel: (Sender<ResponseEvent>, Receiver<ResponseEvent>))
               -> GetAccountInfoResponseGetter {
        GetAccountInfoResponseGetter { data_channel: data_channel }
    }

    /// Get result from the network as informed by MessageQueue. This is blocking. Tuple fields of
    /// result are `(data_stored, space_available)`. `data_stored` means number of chunks Put.
    /// `space_available` means number of chunks which can still be Put.
    pub fn get(&self) -> Result<(u64, u64), CoreError> {
        let (ref sender, ref data_receiver) = self.data_channel;
        let sender = sender.clone();
        let (tx, rx) = mpsc::channel();
        let _thread_canceller = ThreadCanceller(tx);
        wait_canceller(sender, rx);
        let res = data_receiver.recv();
        match try!(res) {
            ResponseEvent::GetAccountInfoResp(result) => result,
            ResponseEvent::Terminated => Err(CoreError::RequestTimeout),
            _ => Err(CoreError::ReceivedUnexpectedData),
        }
    }

    /// Extract associated sender. This will help cancel the blocking wait at will if so desired.
    /// All that is needed is to extract the sender before doing a `get()` and then while blocking
    /// on `get()` fire `sender.send(ResponseEvent::Terminated)` to gracefully exit the receiver.
    pub fn get_sender(&self) -> &Sender<ResponseEvent> {
        &self.data_channel.0
    }
}

/// MutationResponseGetter is a lazy evaluated response getter for mutating network requests such
/// as PUT/POST/DELETE. It will fetch either from local cache or wait for the MessageQueue to notify
/// it of the incoming response from the network
pub struct MutationResponseGetter {
    data_channel: (Sender<ResponseEvent>, Receiver<ResponseEvent>),
}

impl MutationResponseGetter {
    /// Create a new instance of MutationResponseGetter
    pub fn new(data_channel: (Sender<ResponseEvent>, Receiver<ResponseEvent>))
               -> MutationResponseGetter {
        MutationResponseGetter { data_channel: data_channel }
    }

    /// Get response when it comes from the network as informed by MessageQueue. This is blocking
    pub fn get(&self) -> Result<(), CoreError> {
        let (ref sender, ref data_receiver) = self.data_channel;
        let sender = sender.clone();
        let (tx, rx) = mpsc::channel();
        let _thread_canceller = ThreadCanceller(tx);
        wait_canceller(sender, rx);
        match try!(data_receiver.recv()) {
            ResponseEvent::MutationResp(result) => result,
            ResponseEvent::Terminated => Err(CoreError::RequestTimeout),
            _ => Err(CoreError::ReceivedUnexpectedData),
        }
    }

    /// Extract associated sender. This will help cancel the blocking wait at will if so desired.
    /// All that is needed is to extract the sender before doing a `get()` and then while blocking
    /// on `get()` fire `sender.send(ResponseEvent::Terminated)` to gracefully exit the receiver.
    pub fn get_sender(&self) -> &Sender<ResponseEvent> {
        &self.data_channel.0
    }
}

fn wait_canceller(tx: Sender<ResponseEvent>, rx: Receiver<()>) {
    maidsafe_utilities::thread::named("DetachedCanceller", move || {
            const SLEEP_PER_ITER: u64 = 1;
            for _ in 0..REQ_TIMEOUT_SECS {
                if let Ok(()) = rx.try_recv() {
                    return;
                }
                thread::sleep(Duration::from_secs(SLEEP_PER_ITER));
            }
            debug!("Response has timed out - firing wait canceller.");
            let _ = tx.send(ResponseEvent::Terminated);
        })
        .detach();
}

struct ThreadCanceller(Sender<()>);
impl Drop for ThreadCanceller {
    fn drop(&mut self) {
        let _ = self.0.send(());
    }
}
