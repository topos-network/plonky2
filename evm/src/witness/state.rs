use ethereum_types::U256;

use crate::cpu::kernel::aggregator::KERNEL;

const KERNEL_CONTEXT: usize = 0;

/// Structure storing the current register values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegistersState {
    /// Current state of the program counter.
    pub program_counter: usize,
    /// 1 if we are in kernel mode, 0 otherwise.
    pub is_kernel: bool,
    /// Current value of the stack.
    pub stack_len: usize,
    /// Current value stored in the top of the stack.
    pub stack_top: U256,
    /// Indicates if you read the new stack_top from memory to set the channel accordingly.
    pub is_stack_top_read: bool,
    /// Current context.
    pub context: usize,
    /// Current value of the gas used.
    pub gas_used: u64,
}

impl RegistersState {
    /// Returns the context of the kernel if we are in kernel mode,
    /// otherwise returns the current context value.
    pub(crate) fn code_context(&self) -> usize {
        if self.is_kernel {
            KERNEL_CONTEXT
        } else {
            self.context
        }
    }
}

impl Default for RegistersState {
    fn default() -> Self {
        Self {
            program_counter: KERNEL.global_labels["main"],
            is_kernel: true,
            stack_len: 0,
            stack_top: U256::zero(),
            is_stack_top_read: false,
            context: 0,
            gas_used: 0,
        }
    }
}
