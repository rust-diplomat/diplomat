``locale::ffi``
===============

.. js:class:: ICU4XLocale

    An ICU4X Locale, capable of representing strings like ``"en-US"``.

    See the `Rust documentation for Locale <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.


    .. js:staticfunction:: new(name)

        Construct an :js:class:`ICU4XLocale` from a locale identifier represented as a string.


    .. js:staticfunction:: new_from_bytes(bytes)

        Construct an :js:class:`ICU4XLocale` from a locale identifier represented as bytes.

        - Note: ``bytes`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

