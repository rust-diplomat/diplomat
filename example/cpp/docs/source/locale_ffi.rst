``locale::ffi``
===============

.. cpp:class:: Locale

    An  Locale, capable of representing strings like ``"en-US"``.

    See the `Rust documentation for Locale <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.


    .. cpp:function:: static Locale new_(const std::string_view name)

        Construct an :cpp:class:`Locale` from a locale identifier represented as a string.

