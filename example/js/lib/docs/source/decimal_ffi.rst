``decimal::ffi``
================

.. js:class:: FixedDecimalFormatter

    An  Fixed Decimal Format object, capable of formatting a :js:class:`FixedDecimal` as a string.

    See the `Rust documentation for FixedDecimalFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html>`__ for more information.


    .. js:function:: try_new(locale, provider, options)

        Creates a new :js:class:`FixedDecimalFormatter` from locale data.

        See the `Rust documentation for try_new <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new>`__ for more information.


    .. js:method:: format_write(value)

        Formats a :js:class:`FixedDecimal` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format>`__ for more information.


.. js:class:: FixedDecimalFormatterOptions

    .. js:attribute:: grouping_strategy

    .. js:attribute:: some_other_config

    .. js:function:: default()

.. js:class:: FixedDecimalGroupingStrategy
