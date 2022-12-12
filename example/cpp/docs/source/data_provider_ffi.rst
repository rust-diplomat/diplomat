``data_provider::ffi``
======================

.. cpp:class:: ICU4XDataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source.

    See the `Rust documentation for icu_provider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html>`__ for more information.


    .. cpp:function:: static ICU4XDataProvider new_static()

        See the `Rust documentation for get_static_provider <https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/fn.get_static_provider.html>`__ for more information.


    .. cpp:function:: static diplomat::result<std::monostate, std::monostate> returns_result()

        This exists as a regression test for https://github.com/rust-diplomat/diplomat/issues/155

