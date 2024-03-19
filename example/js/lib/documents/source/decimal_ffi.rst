``decimal::ffi``
================

.. js:class:: ICU4XFixedDecimalFormatter

    An ICU4X Fixed Decimal Format object, capable of formatting a :js:class:`ICU4XFixedDecimal` as a string.

    See the `Rust documentation for FixedDecimalFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html>`__ for more information.


    .. js:function:: try_new(locale, provider, options)

        Creates a new :js:class:`ICU4XFixedDecimalFormatter` from locale data.

        See the `Rust documentation for try_new <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new>`__ for more information.


    .. js:method:: format_write(value)

        Formats a :js:class:`ICU4XFixedDecimal` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format>`__ for more information.


.. js:class:: ICU4XFixedDecimalFormatterOptions

    .. js:attribute:: grouping_strategy

    .. js:attribute:: some_other_config

    .. js:function:: default()

.. js:class:: ICU4XFixedDecimalGroupingStrategy
