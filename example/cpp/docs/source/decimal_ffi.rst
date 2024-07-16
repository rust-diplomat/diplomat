``decimal::ffi``
================

.. cpp:class:: FixedDecimalFormatter

    An  Fixed Decimal Format object, capable of formatting a :cpp:class:`FixedDecimal` as a string.

    See the `Rust documentation for FixedDecimalFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<FixedDecimalFormatter, std::monostate> try_new(const Locale& locale, const DataProvider& provider, FixedDecimalFormatterOptions options)

        Creates a new :cpp:class:`FixedDecimalFormatter` from locale data.

        See the `Rust documentation for try_new <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new>`__ for more information.



    .. cpp:function:: template<typename W> void format_write_to_write(const FixedDecimal& value, W& write) const

        Formats a :cpp:class:`FixedDecimal` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format>`__ for more information.


    .. cpp:function:: std::string format_write(const FixedDecimal& value) const

        Formats a :cpp:class:`FixedDecimal` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format>`__ for more information.


.. cpp:struct:: FixedDecimalFormatterOptions

    .. cpp:member:: FixedDecimalGroupingStrategy grouping_strategy

    .. cpp:member:: bool some_other_config

    .. cpp:function:: static FixedDecimalFormatterOptions default_()


.. cpp:enum-struct:: FixedDecimalGroupingStrategy

    .. cpp:enumerator:: Auto

        Auto grouping


    .. cpp:enumerator:: Never

        No grouping


    .. cpp:enumerator:: Always

        Always group


    .. cpp:enumerator:: Min2

        At least 2 groups

