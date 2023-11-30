/// These metadata fields contain global VM state, stored in the `Segment::Metadata` segment of the
/// kernel's context (which is zero).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub(crate) enum GlobalMetadata {
    /// The largest context ID that has been used so far in this execution. Tracking this allows us
    /// give each new context a unique ID, so that its memory will be zero-initialized.
    LargestContext = 0,
    /// The size of active memory, in bytes.
    MemorySize = 1,
    /// The size of the `TrieData` segment, in bytes. In other words, the next address available for
    /// appending additional trie data.
    TrieDataSize = 2,
    /// The size of the `TrieData` segment, in bytes. In other words, the next address available for
    /// appending additional trie data.
    RlpDataSize = 3,
    /// A pointer to the root of the state trie within the `TrieData` buffer.
    StateTrieRoot = 4,
    /// A pointer to the root of the receipt trie within the `TrieData` buffer.
    ReceiptTrieRoot = 5,

    // The root digests of each Merkle trie before these transactions.
    StateTrieRootDigestBefore = 6,
    ReceiptTrieRootDigestBefore = 7,

    // The root digests of each Merkle trie after these transactions.
    StateTrieRootDigestAfter = 8,
    ReceiptTrieRootDigestAfter = 9,

    /// The sizes of the `TrieEncodedChild` and `TrieEncodedChildLen` buffers. In other words, the
    /// next available offset in these buffers.
    TrieEncodedChildSize = 10,

    // Block metadata.
    BlockBeneficiary = 11,
    BlockTimestamp = 12,
    BlockNumber = 13,
    BlockDifficulty = 14,
    BlockRandom = 15,
    BlockGasLimit = 16,
    BlockChainId = 17,
    BlockBaseFee = 18,
    BlockGasUsed = 19,
    /// Before current transactions block values.
    BlockGasUsedBefore = 20,
    /// After current transactions block values.
    BlockGasUsedAfter = 21,
    /// Current block header hash
    BlockCurrentHash = 22,

    /// Gas to refund at the end of the transaction.
    RefundCounter = 23,
    /// Length of the addresses access list.
    AccessedAddressesLen = 24,
    /// Length of the storage keys access list.
    AccessedStorageKeysLen = 25,
    /// Length of the self-destruct list.
    SelfDestructListLen = 26,
    /// Length of the bloom entry buffer.
    BloomEntryLen = 27,

    /// Length of the journal.
    JournalLen = 28,
    /// Length of the `JournalData` segment.
    JournalDataLen = 29,
    /// Current checkpoint.
    CurrentCheckpoint = 30,
    TouchedAddressesLen = 31,
    // Gas cost for the access list in type-1 txns. See EIP-2930.
    AccessListDataCost = 32,
    // Start of the access list in the RLP for type-1 txns.
    AccessListRlpStart = 33,
    // Length of the access list in the RLP for type-1 txns.
    AccessListRlpLen = 34,
    // Boolean flag indicating if the txn is a contract creation txn.
    ContractCreation = 35,
    IsPrecompileFromEoa = 36,
    CallStackDepth = 37,
    /// Transaction logs list length
    LogsLen = 38,
    LogsDataLen = 39,
    LogsPayloadLen = 40,
    TxnNumberBefore = 41,
    TxnNumberAfter = 42,

    KernelHash = 43,
    KernelLen = 44,
}

impl GlobalMetadata {
    pub(crate) const COUNT: usize = 45;

    pub(crate) fn all() -> [Self; Self::COUNT] {
        [
            Self::LargestContext,
            Self::MemorySize,
            Self::TrieDataSize,
            Self::RlpDataSize,
            Self::StateTrieRoot,
            Self::ReceiptTrieRoot,
            Self::StateTrieRootDigestBefore,
            Self::ReceiptTrieRootDigestBefore,
            Self::StateTrieRootDigestAfter,
            Self::ReceiptTrieRootDigestAfter,
            Self::TrieEncodedChildSize,
            Self::BlockBeneficiary,
            Self::BlockTimestamp,
            Self::BlockNumber,
            Self::BlockDifficulty,
            Self::BlockRandom,
            Self::BlockGasLimit,
            Self::BlockChainId,
            Self::BlockBaseFee,
            Self::BlockGasUsed,
            Self::BlockGasUsedBefore,
            Self::BlockGasUsedAfter,
            Self::RefundCounter,
            Self::AccessedAddressesLen,
            Self::AccessedStorageKeysLen,
            Self::SelfDestructListLen,
            Self::BloomEntryLen,
            Self::JournalLen,
            Self::JournalDataLen,
            Self::CurrentCheckpoint,
            Self::TouchedAddressesLen,
            Self::AccessListDataCost,
            Self::AccessListRlpStart,
            Self::AccessListRlpLen,
            Self::ContractCreation,
            Self::IsPrecompileFromEoa,
            Self::CallStackDepth,
            Self::LogsLen,
            Self::LogsDataLen,
            Self::LogsPayloadLen,
            Self::BlockCurrentHash,
            Self::TxnNumberBefore,
            Self::TxnNumberAfter,
            Self::KernelHash,
            Self::KernelLen,
        ]
    }

    /// The variable name that gets passed into kernel assembly code.
    pub(crate) fn var_name(&self) -> &'static str {
        match self {
            Self::LargestContext => "GLOBAL_METADATA_LARGEST_CONTEXT",
            Self::MemorySize => "GLOBAL_METADATA_MEMORY_SIZE",
            Self::TrieDataSize => "GLOBAL_METADATA_TRIE_DATA_SIZE",
            Self::RlpDataSize => "GLOBAL_METADATA_RLP_DATA_SIZE",
            Self::StateTrieRoot => "GLOBAL_METADATA_STATE_TRIE_ROOT",
            Self::ReceiptTrieRoot => "GLOBAL_METADATA_RECEIPT_TRIE_ROOT",
            Self::StateTrieRootDigestBefore => "GLOBAL_METADATA_STATE_TRIE_DIGEST_BEFORE",
            Self::ReceiptTrieRootDigestBefore => "GLOBAL_METADATA_RECEIPT_TRIE_DIGEST_BEFORE",
            Self::StateTrieRootDigestAfter => "GLOBAL_METADATA_STATE_TRIE_DIGEST_AFTER",
            Self::ReceiptTrieRootDigestAfter => "GLOBAL_METADATA_RECEIPT_TRIE_DIGEST_AFTER",
            Self::TrieEncodedChildSize => "GLOBAL_METADATA_TRIE_ENCODED_CHILD_SIZE",
            Self::BlockBeneficiary => "GLOBAL_METADATA_BLOCK_BENEFICIARY",
            Self::BlockTimestamp => "GLOBAL_METADATA_BLOCK_TIMESTAMP",
            Self::BlockNumber => "GLOBAL_METADATA_BLOCK_NUMBER",
            Self::BlockDifficulty => "GLOBAL_METADATA_BLOCK_DIFFICULTY",
            Self::BlockRandom => "GLOBAL_METADATA_BLOCK_RANDOM",
            Self::BlockGasLimit => "GLOBAL_METADATA_BLOCK_GAS_LIMIT",
            Self::BlockChainId => "GLOBAL_METADATA_BLOCK_CHAIN_ID",
            Self::BlockBaseFee => "GLOBAL_METADATA_BLOCK_BASE_FEE",
            Self::BlockGasUsed => "GLOBAL_METADATA_BLOCK_GAS_USED",
            Self::BlockGasUsedBefore => "GLOBAL_METADATA_BLOCK_GAS_USED_BEFORE",
            Self::BlockGasUsedAfter => "GLOBAL_METADATA_BLOCK_GAS_USED_AFTER",
            Self::BlockCurrentHash => "GLOBAL_METADATA_BLOCK_CURRENT_HASH",
            Self::RefundCounter => "GLOBAL_METADATA_REFUND_COUNTER",
            Self::AccessedAddressesLen => "GLOBAL_METADATA_ACCESSED_ADDRESSES_LEN",
            Self::AccessedStorageKeysLen => "GLOBAL_METADATA_ACCESSED_STORAGE_KEYS_LEN",
            Self::SelfDestructListLen => "GLOBAL_METADATA_SELFDESTRUCT_LIST_LEN",
            Self::BloomEntryLen => "GLOBAL_METADATA_BLOOM_ENTRY_LEN",
            Self::JournalLen => "GLOBAL_METADATA_JOURNAL_LEN",
            Self::JournalDataLen => "GLOBAL_METADATA_JOURNAL_DATA_LEN",
            Self::CurrentCheckpoint => "GLOBAL_METADATA_CURRENT_CHECKPOINT",
            Self::TouchedAddressesLen => "GLOBAL_METADATA_TOUCHED_ADDRESSES_LEN",
            Self::AccessListDataCost => "GLOBAL_METADATA_ACCESS_LIST_DATA_COST",
            Self::AccessListRlpStart => "GLOBAL_METADATA_ACCESS_LIST_RLP_START",
            Self::AccessListRlpLen => "GLOBAL_METADATA_ACCESS_LIST_RLP_LEN",
            Self::ContractCreation => "GLOBAL_METADATA_CONTRACT_CREATION",
            Self::IsPrecompileFromEoa => "GLOBAL_METADATA_IS_PRECOMPILE_FROM_EOA",
            Self::CallStackDepth => "GLOBAL_METADATA_CALL_STACK_DEPTH",
            Self::LogsLen => "GLOBAL_METADATA_LOGS_LEN",
            Self::LogsDataLen => "GLOBAL_METADATA_LOGS_DATA_LEN",
            Self::LogsPayloadLen => "GLOBAL_METADATA_LOGS_PAYLOAD_LEN",
            Self::TxnNumberBefore => "GLOBAL_METADATA_TXN_NUMBER_BEFORE",
            Self::TxnNumberAfter => "GLOBAL_METADATA_TXN_NUMBER_AFTER",
            Self::KernelHash => "GLOBAL_METADATA_KERNEL_HASH",
            Self::KernelLen => "GLOBAL_METADATA_KERNEL_LEN",
        }
    }
}
