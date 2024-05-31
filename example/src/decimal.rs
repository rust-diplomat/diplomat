#![allow(clippy::result_unit_err, clippy::should_implement_trait)]

#[diplomat::bridge]
pub mod ffi {
    use diplomat_runtime::DiplomatWrite;
    use icu::decimal::{options::GroupingStrategy, FixedDecimalFormatter};
    use icu_provider::DataLocale;
    use writeable::Writeable;

    use crate::{
        data_provider::ffi::ICU4XDataProvider, fixed_decimal::ffi::ICU4XFixedDecimal,
        locale::ffi::ICU4XLocale,
    };

    #[diplomat::opaque]
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
    #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter, Struct)]
    pub struct ICU4XFixedDecimalFormatter(pub FixedDecimalFormatter);

    // Doc comments for testing TSDoc
    #[diplomat::enum_convert(GroupingStrategy, needs_wildcard)]
    pub enum ICU4XFixedDecimalGroupingStrategy {
        /// Auto grouping
        Auto,
        /// No grouping
        Never,
        /// Always group
        Always,
        /// At least 2 groups
        Min2,
    }

    pub struct ICU4XFixedDecimalFormatterOptions {
        pub grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        pub some_other_config: bool,
    }

    impl ICU4XFixedDecimalFormatterOptions {
        #[diplomat::attr(supports = constructors, constructor)]
        pub fn default() -> Self {
            Self {
                grouping_strategy: ICU4XFixedDecimalGroupingStrategy::Auto,
                some_other_config: false,
            }
        }
    }

    impl ICU4XFixedDecimalFormatter {
        /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::try_new, FnInStruct)]
        // TODO constructors: this should ideally be a constructor too
        pub fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            options: ICU4XFixedDecimalFormatterOptions,
        ) -> Result<Box<ICU4XFixedDecimalFormatter>, ()> {
            let locale = DataLocale::from(locale.0.as_ref());
            let provider = provider.0.as_ref();
            FixedDecimalFormatter::try_new_with_any_provider(provider, &locale, options.into())
                .map_err(|_| ())
                .map(|x| Box::new(ICU4XFixedDecimalFormatter(x)))
        }

        /// Formats a [`ICU4XFixedDecimal`] to a string.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::format, FnInStruct)]
        pub fn format_write(&self, value: &ICU4XFixedDecimal, write: &mut DiplomatWrite) {
            self.0.format(&value.0).write_to(write).unwrap();
            write.flush();
        }
    }
}

impl From<ffi::ICU4XFixedDecimalFormatterOptions>
    for icu::decimal::options::FixedDecimalFormatterOptions
{
    fn from(other: ffi::ICU4XFixedDecimalFormatterOptions) -> Self {
        let mut ret = Self::default();
        ret.grouping_strategy = other.grouping_strategy.into();
        ret
    }
}
