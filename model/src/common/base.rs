pub trait BaseDataModel {}

/// implements [BaseDataModel]
#[macro_export]
macro_rules! bdm {
    ($struct_name:ident) => {
        impl crate::BaseDataModel for $struct_name {}
    };
}
