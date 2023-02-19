use bytes::{Bytes, BytesMut};

use crate::network::raknet::packets::{Ack, Nak};
use crate::network::session::session::Session;
use common::VResult;
use common::{Deserialize, Serialize};

impl Session {
    /// Processes an acknowledgement received from the client.
    ///
    /// This function unregisters the specified packet IDs from the recovery queue.
    pub fn handle_ack(&self, pk: Bytes) -> VResult<()> {
        let ack = Ack::deserialize(pk)?;
        self.recovery_queue.confirm(&ack.records);

        Ok(())
    }

    /// Processes a negative acknowledgement received from the client.
    ///
    /// This function makes sure the packet is retrieved from the recovery queue and sent to the
    /// client again.
    pub async fn handle_nack(&self, pk: Bytes) -> VResult<()> {
        let nack = Nak::deserialize(pk)?;
        let frame_batches = self.recovery_queue.recover(&nack.records);
        tracing::info!("Recovered packets: {:?}", nack.records);

        for frame_batch in frame_batches {
            self.ipv4_socket
                .send_to(frame_batch.serialize()?.as_ref(), self.address)
                .await?;
        }

        Ok(())
    }
}
