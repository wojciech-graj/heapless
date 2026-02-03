//! Bin-proto implementations for heapless types

use crate::{
    len_type::LenType,
    vec::{VecInner, VecStorage, VecStorageInner},
};
use bin_proto::{BitDecode, BitEncode, BitRead, BitWrite, Endianness, Result, Untagged};
use core::mem::MaybeUninit;

impl<Ctx, T, LenT: LenType, S: VecStorage<T> + ?Sized> BitEncode<Ctx, Untagged>
    for VecInner<T, LenT, S>
where
    T: BitEncode<Ctx>,
{
    fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, tag: Untagged) -> Result<()>
    where
        W: BitWrite,
        E: Endianness,
    {
        <[T] as bin_proto::BitEncode<_, _>>::encode::<_, E>(self.as_slice(), write, ctx, tag)
    }
}

impl<Ctx, T, LenT: LenType, const N: usize> BitDecode<Ctx, Untagged>
    for VecInner<T, LenT, VecStorageInner<[MaybeUninit<T>; N]>>
where
    T: BitDecode<Ctx>,
{
    fn decode<R, E>(read: &mut R, ctx: &mut Ctx, _tag: Untagged) -> Result<Self>
    where
        R: BitRead,
        E: Endianness,
    {
        let mut values = Self::new();

        for item in bin_proto::util::decode_items_to_eof::<_, E, _, _>(read, ctx) {
            values
                .push(item?)
                .map_err(|_| bin_proto::Error::Other("insufficient capacity"))?;
        }

        Ok(values)
    }
}
