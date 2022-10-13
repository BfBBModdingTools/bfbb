use std::borrow::Cow;

use bytemuck::CheckedBitPattern;
use process_memory::{Architecture, CopyAddress, Memory, ProcessHandle, PutAddress};

use crate::endian::EndianAware;

const GCN_BASE_ADDRESS: usize = 0x80000000;

/// A specialized version of `DataMember` from the `process_memory` crate,
/// meant for reading/writing to emulated GameCube memory within Dolphin.
///
/// Offsets are constructed as if they were in the GameCube's memory space.
pub(crate) struct DataMember<T> {
    offsets: Vec<usize>,
    process: ProcessHandle,
    emulated_region_address: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> DataMember<T> {
    #[must_use]
    pub fn new_offset(
        handle: ProcessHandle,
        emulated_region_address: usize,
        offsets: Vec<usize>,
    ) -> Self {
        Self {
            offsets,
            process: handle,
            emulated_region_address,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Provides a safe variant of `Memory::read` that will fail if it encounters an invalid bit-pattern
pub trait CheckedMemory<T>: Memory<T> {
    fn checked_read(&self) -> std::io::Result<T>;
}

impl<T: Copy + EndianAware> DataMember<T> {
    /// Read the pointed at value into a byte buffer and return it as-is.
    fn read_bytes(&self) -> std::io::Result<Vec<u8>> {
        let offset = self.get_offset()?;
        let mut buffer = vec![0u8; std::mem::size_of::<T>()];
        self.process.copy_address(offset, &mut buffer)?;
        if T::NEEDS_SWAP {
            buffer.reverse();
        }
        Ok(buffer)
    }
}

impl<T: Copy + EndianAware> Memory<T> for DataMember<T> {
    fn set_offset(&mut self, new_offsets: Vec<usize>) {
        self.offsets = new_offsets;
    }

    fn get_offset(&self) -> std::io::Result<usize> {
        // We cannot call `self.process.get_offset` as it assumes that the
        // endianness of the pointers it traverses are the same as the host
        // system, which is not the case with Dolphin running on a little endian system.

        let mut offset = 0;
        let noffsets = self.offsets.len();
        let mut copy = [0u8; Architecture::Arch32Bit as usize];
        for next_offset in self.offsets.iter().take(noffsets - 1) {
            offset += next_offset;
            offset = offset.checked_sub(GCN_BASE_ADDRESS).ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Attempt to dereference an invalid pointer.",
                )
            })?;
            offset += self.emulated_region_address;
            self.process.copy_address(offset, &mut copy)?;
            offset = u32::from_be_bytes(copy) as usize;
        }

        offset += self.offsets[noffsets - 1];
        offset = offset.checked_sub(GCN_BASE_ADDRESS).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Attempt to dereference an invalid pointer.",
            )
        })?;
        offset += self.emulated_region_address;
        Ok(offset)
    }

    unsafe fn read(&self) -> std::io::Result<T> {
        let buffer = self.read_bytes()?;
        Ok((buffer.as_ptr() as *const T).read_unaligned())
    }

    fn write(&self, value: &T) -> std::io::Result<()> {
        use std::slice;
        let offset = self.get_offset()?;
        let mut buffer = Cow::Borrowed(unsafe {
            slice::from_raw_parts(value as *const _ as _, std::mem::size_of::<T>())
        });
        if T::NEEDS_SWAP {
            buffer.to_mut().reverse();
        }
        self.process.put_address(offset, &buffer)
    }
}

impl<T: CheckedBitPattern + EndianAware> CheckedMemory<T> for DataMember<T> {
    fn checked_read(&self) -> std::io::Result<T> {
        let buffer = self.read_bytes()?;
        let val = bytemuck::checked::try_from_bytes(&buffer[..])
            .map_err(|_| std::io::ErrorKind::InvalidData)?;
        Ok(*val)
    }
}
