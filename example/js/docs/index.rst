
.. js:class:: ICU4XDataProvider


    An ICU4X data provider, capable of loading ICU4X data keys from some source. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html>`__ for more information.

    .. js:staticfunction:: new_static

        Construct a `StaticDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html>`__.

.. js:class:: ICU4XFixedDecimal


    A decimal number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:staticfunction:: new

        Construct an :js:class:`ICU4XFixedDecimal` from an integer.

    .. js:function:: multiply_pow10

        Multiply the :js:class:`ICU4XFixedDecimal` by a given power of ten. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. js:function:: negate

        Invert the sign of the :js:class:`ICU4XFixedDecimal`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. js:function:: to_string

        Format the :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

.. js:class:: ICU4XFixedDecimalFormat


    An ICU4X Fixed Decimal Format object, capable of formatting a :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`__ for more information.

    .. js:staticfunction:: try_new

        Creates a new :js:class:`ICU4XFixedDecimalFormat` from locale data. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new>`__ for more information.

    .. js:function:: format_write

        Formats a :js:class:`ICU4XFixedDecimal` to a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format>`__ for more information.

.. js:class:: ICU4XFixedDecimalFormatResult



    .. js:function:: fdf


        The :js:class:`ICU4XFixedDecimalFormat`, valid if creation was successful.

    .. js:function:: success


        Whether creating the :js:class:`ICU4XFixedDecimalFormat` was successful.

.. js:class:: ICU4XLocale


    An ICU4X Locale, capable of representing strings like ``"en-US"``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

    .. js:staticfunction:: new

        Construct an :js:class:`ICU4XLocale` from an locale identifier.
