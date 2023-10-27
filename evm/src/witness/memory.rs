use ethereum_types::U256;

use crate::cpu::membus::{NUM_CHANNELS, NUM_GP_CHANNELS};

/// Enumerates all memory channels.
#[derive(Clone, Copy, Debug)]
pub enum MemoryChannel {
    /// Memory channel for the code.
    Code,
    /// General purpose memory channels.
    /// There are `NUM_GP_CHANNELS` such channels.
    GeneralPurpose(usize),
}

use MemoryChannel::{Code, GeneralPurpose};

use crate::cpu::kernel::constants::global_metadata::GlobalMetadata;
use crate::memory::segments::Segment;
use crate::witness::errors::MemoryError::{ContextTooLarge, SegmentTooLarge, VirtTooLarge};
use crate::witness::errors::ProgramError;
use crate::witness::errors::ProgramError::MemoryError;

impl MemoryChannel {
    /// Returns the index of the current memory channel.
    pub fn index(&self) -> usize {
        match *self {
            Code => 0,
            GeneralPurpose(n) => {
                assert!(n < NUM_GP_CHANNELS);
                n + 1
            }
        }
    }
}

/// Structure for a memory channel's address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MemoryAddress {
    /// Context of the memory channel.
    pub(crate) context: usize,
    /// Segment of the memory channel.
    pub(crate) segment: usize,
    /// Virtual address of the memory channel.
    pub(crate) virt: usize,
}

impl MemoryAddress {
    /// Returns a new `MemoryAddress` given the context, segment
    /// and virtual address.
    pub(crate) fn new(context: usize, segment: Segment, virt: usize) -> Self {
        Self {
            context,
            segment: segment as usize,
            virt,
        }
    }

    /// Returns a new `MemoryAddress` given the context, segment
    /// and virtual address as U256 words.
    /// Errors if any of the three elements are more than 32-bits long.
    pub(crate) fn new_u256s(
        context: U256,
        segment: U256,
        virt: U256,
    ) -> Result<Self, ProgramError> {
        if context.bits() > 32 {
            return Err(MemoryError(ContextTooLarge { context }));
        }
        if segment >= Segment::COUNT.into() {
            return Err(MemoryError(SegmentTooLarge { segment }));
        }
        if virt.bits() > 32 {
            return Err(MemoryError(VirtTooLarge { virt }));
        }

        // Calling `as_usize` here is safe as those have been checked above.
        Ok(Self {
            context: context.as_usize(),
            segment: segment.as_usize(),
            virt: virt.as_usize(),
        })
    }

    /// Increments the virtual address without overflowing.
    pub(crate) fn increment(&mut self) {
        self.virt = self.virt.saturating_add(1);
    }
}

/// Enumerates all possible memory operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryOpKind {
    Read,
    Write,
}

/// Enumerates the elements comprising a memory operation.
#[derive(Clone, Copy, Debug)]
pub struct MemoryOp {
    /// true if this is an actual memory operation, or false if it's a padding row.
    pub filter: bool,
    /// Timestamp at which the operation occurs.
    pub timestamp: usize,
    /// Address at which the operation occurs.
    pub address: MemoryAddress,
    /// Type of memory operation.
    pub kind: MemoryOpKind,
    /// Value that is either read or written.
    pub value: U256,
}

/// Dummy memory operation. It is a read operation with all values
/// set to 0, except for the filer that is set to `false`.
pub static DUMMY_MEMOP: MemoryOp = MemoryOp {
    filter: false,
    timestamp: 0,
    address: MemoryAddress {
        context: 0,
        segment: 0,
        virt: 0,
    },
    kind: MemoryOpKind::Read,
    value: U256::zero(),
};

impl MemoryOp {
    /// Returns a new memory operation given its address, its type,
    /// the clock value at which it was called, and the value that
    /// is either read or written.
    pub fn new(
        channel: MemoryChannel,
        clock: usize,
        address: MemoryAddress,
        kind: MemoryOpKind,
        value: U256,
    ) -> Self {
        let timestamp = clock * NUM_CHANNELS + channel.index();
        MemoryOp {
            filter: true,
            timestamp,
            address,
            kind,
            value,
        }
    }

    /// Creates a new dummy read operation at a given address and timestamp:
    /// the filter is set to 0 but the other fields are filled with the provided values.
    pub(crate) fn new_dummy_read(address: MemoryAddress, timestamp: usize, value: U256) -> Self {
        Self {
            filter: false,
            timestamp,
            address,
            kind: MemoryOpKind::Read,
            value,
        }
    }

    /// Returns the `MemoryOp` elements in the order by which they should be sorted.
    pub(crate) fn sorting_key(&self) -> (usize, usize, usize, usize) {
        (
            self.address.context,
            self.address.segment,
            self.address.virt,
            self.timestamp,
        )
    }
}

/// Structure containing the values stored at the current state of the memory.
#[derive(Clone, Debug)]
pub struct MemoryState {
    /// Vector of context states, containing the values stored at
    /// each context.
    pub(crate) contexts: Vec<MemoryContextState>,
}

impl MemoryState {
    /// Fills the `MemoryState` at context 0 with bytes read from the kernel code.
    pub fn new(kernel_code: &[u8]) -> Self {
        let code_u256s = kernel_code.iter().map(|&x| x.into()).collect();
        let mut result = Self::default();
        result.contexts[0].segments[Segment::Code as usize].content = code_u256s;
        result
    }

    /// Applies all memory operations to `MemoryState`.
    pub fn apply_ops(&mut self, ops: &[MemoryOp]) {
        for &op in ops {
            let MemoryOp {
                address,
                kind,
                value,
                ..
            } = op;
            if kind == MemoryOpKind::Write {
                self.set(address, value);
            }
        }
    }

    /// Returns the value stored at a given address.
    pub fn get(&self, address: MemoryAddress) -> U256 {
        if address.context >= self.contexts.len() {
            return U256::zero();
        }

        let segment = Segment::all()[address.segment];
        let val = self.contexts[address.context].segments[address.segment].get(address.virt);
        assert!(
            val.bits() <= segment.bit_range(),
            "Value {} exceeds {:?} range of {} bits",
            val,
            segment,
            segment.bit_range()
        );
        val
    }

    /// Sets the value at a given address in `MemoryContextState`
    /// to the provided `val`.
    pub fn set(&mut self, address: MemoryAddress, val: U256) {
        while address.context >= self.contexts.len() {
            self.contexts.push(MemoryContextState::default());
        }

        let segment = Segment::all()[address.segment];
        assert!(
            val.bits() <= segment.bit_range(),
            "Value {} exceeds {:?} range of {} bits",
            val,
            segment,
            segment.bit_range()
        );
        self.contexts[address.context].segments[address.segment].set(address.virt, val);
    }

    /// Returns the value stored at context 0, segment `GlobalMetadata`
    /// and virtual address `field`.
    pub(crate) fn read_global_metadata(&self, field: GlobalMetadata) -> U256 {
        self.get(MemoryAddress::new(
            0,
            Segment::GlobalMetadata,
            field as usize,
        ))
    }
}

impl Default for MemoryState {
    fn default() -> Self {
        Self {
            // We start with an initial context for the kernel.
            contexts: vec![MemoryContextState::default()],
        }
    }
}

/// Structure comprised of the contents of each segment.
#[derive(Clone, Debug)]
pub(crate) struct MemoryContextState {
    /// The content of each memory segment.
    pub(crate) segments: [MemorySegmentState; Segment::COUNT],
}

impl Default for MemoryContextState {
    fn default() -> Self {
        Self {
            segments: std::array::from_fn(|_| MemorySegmentState::default()),
        }
    }
}

/// Structure comprised of the values contained in a given segment.
#[derive(Clone, Default, Debug)]
pub(crate) struct MemorySegmentState {
    /// Vector of values in a given segment.
    pub(crate) content: Vec<U256>,
}

impl MemorySegmentState {
    /// Returns the value stored at offset `virtual_addr`.
    pub(crate) fn get(&self, virtual_addr: usize) -> U256 {
        self.content
            .get(virtual_addr)
            .copied()
            .unwrap_or(U256::zero())
    }

    /// Sets the value stored at offset `virtual_addr` to `value`.
    pub(crate) fn set(&mut self, virtual_addr: usize, value: U256) {
        if virtual_addr >= self.content.len() {
            self.content.resize(virtual_addr + 1, U256::zero());
        }
        self.content[virtual_addr] = value;
    }
}
