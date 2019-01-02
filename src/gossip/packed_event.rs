// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use super::content::Content;
use super::event_hash::EventHash;
use crate::hash::Hash;
use crate::id::PublicId;
use crate::network_event::NetworkEvent;
use crate::serialise;
use crate::vote::Vote;
use std::fmt::{self, Debug, Formatter};

/// Packed event contains only content and signature.
#[serde(bound = "")]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PackedEvent<T: NetworkEvent, P: PublicId> {
    pub(super) content: Content<Vote<T, P>, EventHash, P>,
    pub(super) signature: P::Signature,
}

impl<T: NetworkEvent, P: PublicId> Debug for PackedEvent<T, P> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "Event{{ {:?}, creator: {:?}, self_parent: {:?}, other_parent: {:?} }}",
            self.content.cause,
            self.content.creator,
            self.content.self_parent(),
            self.content.other_parent()
        )
    }
}

impl<T: NetworkEvent, P: PublicId> PackedEvent<T, P> {
    pub(crate) fn compute_hash(&self) -> EventHash {
        EventHash(Hash::from(serialise(&self.content).as_slice()))
    }
}

#[cfg(feature = "malice-detection")]
impl<T: NetworkEvent, P: PublicId> PackedEvent<T, P> {
    pub(crate) fn self_parent(&self) -> Option<&EventHash> {
        self.content.self_parent()
    }

    pub(crate) fn creator(&self) -> &P {
        &self.content.creator
    }
}
