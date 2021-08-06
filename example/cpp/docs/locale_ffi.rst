``locale::ffi``
===============


.. cpp:class:: ICU4XLocale


    An ICU4X Locale, capable of representing strings like ``"en-US"``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

    .. cpp:function:: static ICU4XLocale new_(const std::string_view name)

        Construct an :cpp:class:`ICU4XLocale` from an locale identifier.
