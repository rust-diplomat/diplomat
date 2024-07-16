``fixed_decimal::ffi``
======================

.. cpp:class:: FixedDecimal

    See the `Rust documentation for FixedDecimal <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html>`__ for more information.


    .. cpp:function:: static FixedDecimal new_(int32_t v)

        Construct an :cpp:class:`FixedDecimal` from an integer.


    .. cpp:function:: void multiply_pow10(int16_t power)

        Multiply the :cpp:class:`FixedDecimal` by a given power of ten.

        See the `Rust documentation for multiply_pow10 <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.



    .. cpp:function:: template<typename W> diplomat::result<std::monostate, std::monostate> to_string_to_write(W& to) const

        Format the :cpp:class:`FixedDecimal` as a string.

        See the `Rust documentation for write_to <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, std::monostate> to_string() const

        Format the :cpp:class:`FixedDecimal` as a string.

        See the `Rust documentation for write_to <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

