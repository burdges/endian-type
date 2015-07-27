#![feature(raw)]

extern crate num;

use std::{mem,ptr,slice};
use std::convert::{From,Into};
use std::ops::{BitAnd,BitOr,BitXor};
use std::raw::Repr;

///Type with a specified byte order
pub trait Endian<T>{}

macro_rules! impl_BitOp{
	( for $e:ident) => {
		impl<T> BitAnd for $e<T>
			where T: BitAnd
		{
			type Output = $e<<T as BitAnd>::Output>;

			fn bitand(self,other: Self) -> Self::Output{
				$e(self.0 & other.0)
			}
		}
		impl<T> BitOr for $e<T>
			where T: BitOr
		{
			type Output = $e<<T as BitOr>::Output>;

			fn bitor(self,other: Self) -> Self::Output{
				$e(self.0 | other.0)
			}
		}
		impl<T> BitXor for $e<T>
			where T: BitXor
		{
			type Output = $e<<T as BitXor>::Output>;

			fn bitxor(self,other: Self) -> Self::Output{
				$e(self.0 ^ other.0)
			}
		}

		impl<T> $e<T>
			where T: Sized + Copy
		{
			pub fn from_bytes(bytes: &[u8]) -> Self{
				debug_assert!(bytes.len() >= mem::size_of::<T>());
				$e(unsafe{*(bytes.repr().data as *const T)})
			}

			pub fn as_bytes(&self) -> &[u8]{
				unsafe{slice::from_raw_parts(
					&self.0 as *const T as *const u8,
					mem::size_of::<T>()
				)}
			}

			/*pub fn write_bytes(self,buffer: &mut [u8]){
				debug_assert!(buffer.len() >= mem::size_of::<T>());
				let bytes = mem::transmute::<_,[u8; mem::size_of::<T>()]>();
				unsafe{ptr::copy_nonoverlapping(bytes.as_ptr(),buffer.as_mut_ptr(),mem::size_of::<T>())};
			}*/
		}
	}
}



///Big endian byte order
///Most significant byte first
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash,Ord,PartialOrd)]
pub struct BigEndian<T>(T);
impl<T> Endian<T> for BigEndian<T>{}
impl<T> From<T> for BigEndian<T>
	where T: num::PrimInt
{
	fn from(data: T) -> Self{
		BigEndian(data.to_be())
	}
}
impl<T> From<LittleEndian<T>> for BigEndian<T>
	where T: num::PrimInt
{
	fn from(data: LittleEndian<T>) -> Self{
		BigEndian(data.0.swap_bytes())
	}
}
macro_rules! impl_Into_for_BigEndian{
	( $t:ident ) => {
		impl Into<$t> for BigEndian<$t>{
			fn into(self) -> $t{
				$t::from_be(self.0)
			}
		}
	}
}
impl_BitOp!(for BigEndian);
impl_Into_for_BigEndian!(u16);
impl_Into_for_BigEndian!(u32);
impl_Into_for_BigEndian!(u64);
impl_Into_for_BigEndian!(usize);
impl_Into_for_BigEndian!(i16);
impl_Into_for_BigEndian!(i32);
impl_Into_for_BigEndian!(i64);
impl_Into_for_BigEndian!(isize);



///Little endian byte order
///Least significant byte first
#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash,Ord,PartialOrd)]
pub struct LittleEndian<T>(T);
impl<T> Endian<T> for LittleEndian<T>{}
impl<T> From<T> for LittleEndian<T>
	where T: num::PrimInt
{
	fn from(data: T) -> Self{
		LittleEndian(data.to_le())
	}
}
impl<T> From<BigEndian<T>> for LittleEndian<T>
	where T: num::PrimInt
{
	fn from(data: BigEndian<T>) -> Self{
		LittleEndian(data.0.swap_bytes())
	}
}
macro_rules! impl_Into_for_LittleEndian{
	( $t:ident ) => {
		impl Into<$t> for LittleEndian<$t>{
			fn into(self) -> $t{
				$t::from_le(self.0)
			}
		}
	}
}
impl_BitOp!(for LittleEndian);
impl_Into_for_LittleEndian!(u16);
impl_Into_for_LittleEndian!(u32);
impl_Into_for_LittleEndian!(u64);
impl_Into_for_LittleEndian!(usize);
impl_Into_for_LittleEndian!(i16);
impl_Into_for_LittleEndian!(i32);
impl_Into_for_LittleEndian!(i64);
impl_Into_for_LittleEndian!(isize);



pub type NetworkOrder<T> = BigEndian<T>;



pub mod types{
	#![allow(non_camel_case_types)]

	use super::*;

	pub type i16_be   = BigEndian<i16>;
	pub type i32_be   = BigEndian<i32>;
	pub type i64_be   = BigEndian<i64>;
	pub type isize_be = BigEndian<isize>;

	pub type u16_be   = BigEndian<u16>;
	pub type u32_be   = BigEndian<u32>;
	pub type u64_be   = BigEndian<u64>;
	pub type usize_be = BigEndian<usize>;

	pub type i16_le   = LittleEndian<i16>;
	pub type i32_le   = LittleEndian<i32>;
	pub type i64_le   = LittleEndian<i64>;
	pub type isize_le = LittleEndian<isize>;

	pub type u16_le   = LittleEndian<u16>;
	pub type u32_le   = LittleEndian<u32>;
	pub type u64_le   = LittleEndian<u64>;
	pub type usize_le = LittleEndian<usize>;

	pub type i16_net   = NetworkOrder<i16>;
	pub type i32_net   = NetworkOrder<i32>;
	pub type i64_net   = NetworkOrder<i64>;
	pub type isize_net = NetworkOrder<isize>;

	pub type u16_net   = NetworkOrder<u16>;
	pub type u32_net   = NetworkOrder<u32>;
	pub type u64_net   = NetworkOrder<u64>;
	pub type usize_net = NetworkOrder<usize>;
}

/*#[cfg(test)]
mod tests{
	use super::*;
	use super::types::*;

	#[test]
	fn construct_big(){
		//#[cfg(target_endian = "big")]{}

	}
}*/