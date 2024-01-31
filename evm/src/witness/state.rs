use ethereum_types::U256;
use serde::{Deserialize, Serialize};

use crate::cpu::kernel::aggregator::KERNEL;

const KERNEL_CONTEXT: usize = 0;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct RegistersState {
    pub program_counter: usize,
    pub is_kernel: bool,
    pub stack_len: usize,
    pub stack_top: U256,
    // Indicates if you read the new stack_top from memory to set the channel accordingly.
    pub is_stack_top_read: bool,
    // Indicates if the previous operation might have caused an overflow, and we must check
    // if it's the case.
    pub check_overflow: bool,
    pub context: usize,
    pub gas_used: u64,
}

impl RegistersState {
    pub(crate) const fn code_context(&self) -> usize {
        if self.is_kernel {
            KERNEL_CONTEXT
        } else {
            self.context
        }
    }

    pub fn new_with_main_label() -> Self {
        Self {
            program_counter: KERNEL.global_labels["main_contd"],
            is_kernel: true,
            stack_len: 0,
            stack_top: U256::zero(),
            is_stack_top_read: false,
            check_overflow: false,
            context: 0,
            gas_used: 0,
        }
    }

    pub fn new_last_registers_with_gas(gas_used: u64) -> Self {
        Self {
            program_counter: KERNEL.global_labels["halt"],
            is_kernel: true,
            stack_len: 0,
            stack_top: U256::zero(),
            is_stack_top_read: false,
            check_overflow: false,
            context: 0,
            gas_used,
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
            check_overflow: false,
            context: 0,
            gas_used: 0,
        }
    }
}
