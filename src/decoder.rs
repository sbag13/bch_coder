use bitvec::BitVec;

pub trait Decoder {
    fn decode(self, encoded: &BitVec) -> Result<(BitVec, BitVec), String>;
}
