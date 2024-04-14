``decimal::ffi``
================

.. cpp:class:: ICU4XFixedDecimalFormatter

    An ICU4X Fixed Decimal Format object, capable of formatting a :cpp:class:`ICU4XFixedDecimal` as a string.

    See the `Rust documentation for FixedDecimalFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XFixedDecimalFormatter, std::monostate> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatterOptions options)

        Creates a new :cpp:class:`ICU4XFixedDecimalFormatter` from locale data.

        See the `Rust documentation for try_new <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new>`__ for more information.



    .. cpp:function:: template<typename W> void format_write_to_writeable(const ICU4XFixedDecimal& value, W& write) const

        Formats a :cpp:class:`ICU4XFixedDecimal` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format>`__ for more information.


    .. cpp:function:: std::string format_write(const ICU4XFixedDecimal& value) const

        Formats a :cpp:class:`ICU4XFixedDecimal` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format>`__ for more information.


.. cpp:struct:: ICU4XFixedDecimalFormatterOptions

    .. cpp:member:: ICU4XFixedDecimalGroupingStrategy grouping_strategy

    .. cpp:member:: bool some_other_config

    .. cpp:function:: static ICU4XFixedDecimalFormatterOptions default_()


.. cpp:enum-struct:: ICU4XFixedDecimalGroupingStrategy

    .. cpp:enumerator:: Auto

        Auto grouping


    .. cpp:enumerator:: Never

        No grouping


    .. cpp:enumerator:: Always

        Always group


    .. cpp:enumerator:: Min2

        At least 2 groups

