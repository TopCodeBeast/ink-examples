#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod storage_types {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[derive(Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        ThisIsAnErrorEnum,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Default, scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum EnumWithoutValues {
        #[default]
        A,
        B,
        C,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum EnumWithValues {
        OneValue(u32),
        TwoValues(u32, u32),
        ThreeValues(u32, u32, u32),
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PrimitiveTypes {
        bool_value: bool,
        enum_without_values: EnumWithoutValues,
        enum_with_values: EnumWithValues,
        array_value: [u32; 3],
        tuple_value: (u32, u32),
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct SignedIntegers {
        i128_value_max: i128,
        i128_value_min: i128,
        i16_value_max: i16,
        i16_value_min: i16,
        i32_value_max: i32,
        i32_value_min: i32,
        i64_value_max: i64,
        i64_value_min: i64,
        i8_value_max: i8,
        i8_value_min: i8,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct SubstrateTypes {
        account_id_value: AccountId,
        balance_value_max: Balance,
        balance_value_min: Balance,
        hash_value: Hash,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct InkPreludeTypes {
        string_value: String,
        vec_string_value: Vec<String>,
        vec_vec_string_value: Vec<Vec<String>>,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct UnsignedIntegers {
        u128_value_max: u128,
        u128_value_min: u128,
        u16_value_max: u16,
        u16_value_min: u16,
        u32_value_max: u32,
        u32_value_min: u32,
        u64_value_max: u64,
        u64_value_min: u64,
        u8_value_max: u8,
        u8_value_min: u8,
    }

    #[derive(Debug)]
    #[ink::storage_item]
    pub struct MappingTypes {
        mapping_u128_u128_value: Mapping<u128, u128>,
        mapping_account_balance_value: Mapping<AccountId, Balance>,
        mapping_account_hash_value: Mapping<AccountId, Hash>,
        mapping_account_account_balance_value: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(storage)]
    pub struct StorageTypes {
        ink_prelude_types: InkPreludeTypes,
        primitive_types: PrimitiveTypes,
        signed_integers: SignedIntegers,
        substrate_types: SubstrateTypes,
        unsigned_integers: UnsignedIntegers,
        mapping_types: MappingTypes,
    }

    impl StorageTypes {
        #[ink(constructor)]
        pub fn new() -> Self {
            // Vectors
            let mut vec_string_value: Vec<String> = Vec::new();
            vec_string_value.push(String::from("This is a string"));
            vec_string_value.push(String::from("This is another string"));

            let mut vec_vec_string_value: Vec<Vec<String>> = Vec::new();
            vec_vec_string_value.push(vec_string_value.clone());

            let mut vec_vec_string_value: Vec<Vec<String>> = Vec::new();
            vec_vec_string_value.push(vec_string_value.clone());

            // Mappings
            let mut mapping_u128_u128_value = Mapping::new();
            mapping_u128_u128_value.insert(42, &23);

            let mut mapping_account_balance_value = Mapping::new();
            mapping_account_balance_value.insert(AccountId::from([0x01; 32]), &42);

            let mut mapping_account_hash_value = Mapping::new();
            mapping_account_hash_value
                .insert(AccountId::from([0x01; 32]), &Hash::from([0xff; 32]));

            let mut mapping_account_account_balance_value = Mapping::new();
            mapping_account_account_balance_value.insert(
                (AccountId::from([0x01; 32]), AccountId::from([0x02; 32])),
                &23,
            );

            Self {
                unsigned_integers: UnsignedIntegers {
                    u128_value_max: u128::MAX,
                    u128_value_min: u128::MIN,
                    u16_value_max: u16::MAX,
                    u16_value_min: u16::MIN,
                    u32_value_max: u32::MAX,
                    u32_value_min: u32::MIN,
                    u64_value_max: u64::MAX,
                    u64_value_min: u64::MIN,
                    u8_value_max: u8::MAX,
                    u8_value_min: u8::MIN,
                },
                signed_integers: SignedIntegers {
                    i128_value_max: i128::MAX,
                    i128_value_min: i128::MIN,
                    i16_value_max: i16::MAX,
                    i16_value_min: i16::MIN,
                    i32_value_max: i32::MAX,
                    i32_value_min: i32::MIN,
                    i64_value_max: i64::MAX,
                    i64_value_min: i64::MIN,
                    i8_value_max: i8::MAX,
                    i8_value_min: i8::MIN,
                },
                ink_prelude_types: InkPreludeTypes {
                    string_value: String::from("This is a string"),
                    vec_string_value,
                    vec_vec_string_value,
                },
                primitive_types: PrimitiveTypes {
                    bool_value: true,
                    enum_with_values: EnumWithValues::ThreeValues(1, 2, 3),
                    enum_without_values: EnumWithoutValues::A,
                    array_value: [3, 2, 1],
                    tuple_value: (7, 8),
                },
                substrate_types: SubstrateTypes {
                    account_id_value: AccountId::from([0x00; 32]),
                    balance_value_max: Balance::MAX,
                    balance_value_min: Balance::MIN,
                    hash_value: Hash::from([0x00; 32]),
                },
                mapping_types: MappingTypes {
                    mapping_u128_u128_value,
                    mapping_account_balance_value,
                    mapping_account_hash_value,
                    mapping_account_account_balance_value,
                },
            }
        }

        #[ink(message)]
        pub fn get_unsigned_integers(&self) -> UnsignedIntegers {
            self.unsigned_integers.clone()
        }

        #[ink(message)]
        pub fn get_signed_integers(&self) -> SignedIntegers {
            self.signed_integers.clone()
        }

        #[ink(message)]
        pub fn get_ink_prelude_types(&self) -> InkPreludeTypes {
            self.ink_prelude_types.clone()
        }

        #[ink(message)]
        pub fn get_substrate_types(&self) -> SubstrateTypes {
            self.substrate_types.clone()
        }

        #[ink(message)]
        pub fn get_primitive_types(&self) -> PrimitiveTypes {
            self.primitive_types.clone()
        }

        #[ink(message)]
        pub fn get_error(&self) -> Result<()> {
            Err(Error::ThisIsAnErrorEnum)
        }
    }

    #[cfg(test)]
    mod tests {}
}
