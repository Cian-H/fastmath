mod lookup_table {
    #[macro_export]
    macro_rules! impl_fbitfbit_lookup_table {
        ($key_type:ty, $value_type:ty) => {
            impl FloatLookupTable<$key_type, $value_type> {
                pub const fn new_const(keys: [$key_type; TABLE_SIZE], values: [$value_type; TABLE_SIZE]) -> Self {
                    FloatLookupTable {
                        keys: keys,
                        max_key: keys[TABLE_SIZE - 1],
                        values: values,
                    }
                }
            }
        };
    }
    
    #[macro_export]
    macro_rules! impl_cycling_fbitfbit_lookup_table {
        ($key_type:ty, $value_type:ty) => {
            impl CyclingFloatLookupTable<$key_type, $value_type> {
                pub const fn new_const(keys: [$key_type; TABLE_SIZE], values: [$value_type; TABLE_SIZE], lower_bound: $key_type, range: $key_type) -> Self {
                    CyclingFloatLookupTable {
                        lookup_table: FloatLookupTable::<$key_type, $value_type>::new_const(keys, values),
                        lower_bound: lower_bound,
                        range: range,
                    }
                }
            }
        };
    }
}