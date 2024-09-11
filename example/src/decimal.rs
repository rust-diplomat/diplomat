#![allow(clippy::result_unit_err, clippy::should_implement_trait)]

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use diplomat_runtime::DiplomatWrite;
    use icu_provider::DataLocale;
    use writeable::Writeable;

    use crate::{
        data_provider::ffi::DataProvider, fixed_decimal::ffi::FixedDecimal, locale::ffi::Locale,
    };

    #[diplomat::opaque]
    /// An  Fixed Decimal Format object, capable of formatting a [`FixedDecimal`] as a string.
    #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter, Struct)]
    pub struct FixedDecimalFormatter(pub icu::decimal::FixedDecimalFormatter);

    // Doc comments for testing TSDoc
    #[diplomat::enum_convert(icu::decimal::options::GroupingStrategy, needs_wildcard)]
    pub enum FixedDecimalGroupingStrategy {
        /// Auto grouping
        Auto,
        /// No grouping
        Never,
        /// Always group
        Always,
        /// At least 2 groups
        Min2,
    }

    pub struct FixedDecimalFormatterOptions {
        #[diplomat::demo(input(label = "ICU4X Fixed Decimal Grouping Strategy"))]
        pub grouping_strategy: FixedDecimalGroupingStrategy,
        #[diplomat::demo(input(label = "Useless Config (Ignore)", default_value = "true"))]
        pub some_other_config: bool,
    }

    impl FixedDecimalFormatterOptions {
        #[diplomat::attr(auto, constructor)]
        pub fn default() -> Self {
            Self {
                grouping_strategy: FixedDecimalGroupingStrategy::Auto,
                some_other_config: false,
            }
        }
    }

    impl FixedDecimalFormatter {
        /// Creates a new [`FixedDecimalFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::try_new, FnInStruct)]
        #[diplomat::demo(default_constructor)]
        // TODO constructors: this should ideally be a constructor too
        pub fn try_new(
            locale: &Locale,
            provider: &DataProvider,
            options: FixedDecimalFormatterOptions,
        ) -> Result<Box<FixedDecimalFormatter>, ()> {
            let locale = DataLocale::from(locale.0.as_ref());
            let provider = provider.0.as_ref();
            icu::decimal::FixedDecimalFormatter::try_new_with_any_provider(
                provider,
                &locale,
                options.into(),
            )
            .map_err(|_| ())
            .map(|x| Box::new(FixedDecimalFormatter(x)))
        }

        /// Formats a [`FixedDecimal`] to a string.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::format, FnInStruct)]
        pub fn format_write(&self, value: &FixedDecimal, write: &mut DiplomatWrite) {
            self.0.format(&value.0).write_to(write).unwrap();
            write.flush();
        }
    }
}

impl From<ffi::FixedDecimalFormatterOptions>
    for icu::decimal::options::FixedDecimalFormatterOptions
{
    fn from(other: ffi::FixedDecimalFormatterOptions) -> Self {
        let mut ret = Self::default();
        ret.grouping_strategy = other.grouping_strategy.into();
        ret
    }
}
