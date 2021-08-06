``fixed_decimal::ffi``
======================


.. cpp:class:: ICU4XFixedDecimal


    A decimal number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: static ICU4XFixedDecimal new_(int32_t v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an integer.

    .. cpp:function:: void multiply_pow10(int16_t power)

        Multiply the :cpp:class:`ICU4XFixedDecimal` by a given power of ten. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. cpp:function:: void negate()

        Invert the sign of the :cpp:class:`ICU4XFixedDecimal`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. cpp:function:: diplomat::result<std::monostate, std::monostate> to_string()

        Format the :cpp:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.
