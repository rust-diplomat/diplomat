``fixed_decimal::ffi``
======================

.. js:class:: ICU4XFixedDecimal

    A decimal number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:staticfunction:: new(v)

        Construct an :js:class:`ICU4XFixedDecimal` from an integer.

    .. js:function:: multiply_pow10(power)

        Multiply the :js:class:`ICU4XFixedDecimal` by a given power of ten. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. js:function:: negate()

        Invert the sign of the :js:class:`ICU4XFixedDecimal`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. js:function:: to_string()

        Format the :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.
