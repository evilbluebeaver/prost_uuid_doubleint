use derive_more::{AsMut, AsRef, Constructor, Deref, DerefMut, From, Into};
use prost::{
    bytes::{Buf, BufMut},
    encoding::{skip_field, uint64, DecodeContext, WireType},
    DecodeError, Message,
};
use uuid::Uuid;
const HIGH_TAG: u32 = 1;
const LOW_TAG: u32 = 2;

#[derive(
    Clone,
    Copy,
    AsRef,
    AsMut,
    Deref,
    DerefMut,
    From,
    Into,
    Constructor,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
)]
pub struct ProstUuid(Uuid);

impl Message for ProstUuid {
    fn encode_raw(&self, buf: &mut impl BufMut) {
        let (high, low) = self.0.as_u64_pair();
        uint64::encode(HIGH_TAG, &high, buf);
        uint64::encode(LOW_TAG, &low, buf);
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut impl Buf,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        Self: Sized,
    {
        match tag {
            HIGH_TAG => {
                let (mut high, low) = self.0.as_u64_pair();
                uint64::merge(wire_type, &mut high, buf, ctx)?;
                self.0 = Uuid::from_u64_pair(high, low);
                Ok(())
            }
            LOW_TAG => {
                let (high, mut low) = self.0.as_u64_pair();
                uint64::merge(wire_type, &mut low, buf, ctx)?;
                self.0 = Uuid::from_u64_pair(high, low);
                Ok(())
            }
            _ => skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        let (high, low) = self.0.as_u64_pair();
        uint64::encoded_len(HIGH_TAG, &high) + uint64::encoded_len(LOW_TAG, &low)
    }

    fn clear(&mut self) {
        self.0 = Uuid::nil();
    }
}
