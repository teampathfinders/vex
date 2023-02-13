use bytes::{BufMut, BytesMut};
use dashmap::DashMap;

use crate::network::raknet::Frame;
use crate::network::raknet::Reliability;

type Fragment = BytesMut;

/// Keeps track of packet fragments, merging them when all fragments have been received.
#[derive(Debug, Default)]
pub struct CompoundCollector {
    compounds: DashMap<u16, Vec<Fragment>>,
}

impl CompoundCollector {
    /// Creates a new collector.
    pub fn new() -> Self {
        Self {
            compounds: DashMap::new(),
        }
    }

    /// Inserts a fragment into the collector.
    ///
    /// If this fragment makes the compound complete, all fragments will be merged
    /// and the completed packet will be returned.
    pub fn insert(&self, mut frame: Frame) -> Option<Frame> {
        let is_completed = {
            let mut entry =
                self.compounds.entry(frame.compound_id).or_insert_with(|| {
                    let mut vec =
                        Vec::with_capacity(frame.compound_size as usize);
                    vec.resize(frame.compound_size as usize, BytesMut::new());
                    vec
                });

            let mut fragments = entry.value_mut();

            // Verify that the fragment index is valid
            if frame.compound_index >= frame.compound_size {
                return None;
            }

            fragments[frame.compound_index as usize] = frame.body.clone();
            !fragments.iter().any(BytesMut::is_empty)
        };

        if is_completed {
            let mut kv = self
                .compounds
                .remove(&frame.compound_id)
                .expect("Compound ID was not found in collector");

            let fragments = &mut kv.1;

            // Calculate total body size
            let total_size = fragments.iter().fold(0, |acc, f| acc + f.len());

            frame.body.clear();
            frame.body.reserve(total_size - frame.body.capacity());

            // Merge all fragments
            for mut fragment in fragments.iter() {
                frame.body.put(fragment.as_ref());
            }

            // Set compound tag to false to make sure the completed packet isn't added into the
            // collector again.
            frame.is_compound = false;
            // Set reliability to unreliable to prevent duplicated acknowledgements
            // frame.reliability = Reliability::Unreliable;

            return Some(frame);
        }

        None
    }
}
