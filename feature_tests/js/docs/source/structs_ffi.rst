``structs::ffi``
================

.. js:class:: ContiguousEnum

.. js:class:: MyEnum

    .. js:method:: into_value()

    .. js:function:: get_a()

.. js:class:: MyStruct

    .. js:attribute:: a

    .. js:attribute:: b

    .. js:attribute:: c

    .. js:attribute:: d

    .. js:attribute:: e

    .. js:attribute:: f

    .. js:attribute:: g

    .. js:function:: new()

    .. js:method:: into_a()

.. js:class:: Opaque

    .. js:function:: new()

    .. js:function:: try_from_utf8(input)

    .. js:function:: from_str(input)

    .. js:method:: get_debug_str()

    .. js:method:: assert_struct(s)

        See the `Rust documentation for something <https://docs.rs/Something/latest/struct.Something.html#method.something>`__ for more information.

        See the `Rust documentation for something_else <https://docs.rs/Something/latest/struct.Something.html#method.something_else>`__ for more information.

        Additional information: `1 <https://docs.rs/Something/latest/struct.Something.html#method.something_small>`__, `2 <https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something>`__


    .. js:function:: returns_usize()

    .. js:function:: returns_imported()

    .. js:function:: cmp()

.. js:class:: OpaqueMutexedString

    .. js:function:: from_usize(number)

    .. js:method:: change(number)

    .. js:method:: get_len_and_add(other)

    .. js:method:: dummy_str()

    .. js:method:: wrapper()

.. js:class:: Utf16Wrap

    .. js:function:: from_utf16(input)

    .. js:method:: get_debug_str()

    .. js:method:: borrow_cont()

    .. js:method:: owned()
