``structs::ffi``
================

.. cpp:enum-struct:: ContiguousEnum

    .. cpp:enumerator:: C

    .. cpp:enumerator:: D

    .. cpp:enumerator:: E

    .. cpp:enumerator:: F

.. cpp:enum-struct:: MyEnum

    .. cpp:enumerator:: A

    .. cpp:enumerator:: B

    .. cpp:enumerator:: C

    .. cpp:enumerator:: D

    .. cpp:enumerator:: E

    .. cpp:enumerator:: F

    .. cpp:function:: int8_t into_value()


.. cpp:struct:: MyStruct

    .. cpp:member:: uint8_t a

    .. cpp:member:: bool b

    .. cpp:member:: uint8_t c

    .. cpp:member:: uint64_t d

    .. cpp:member:: int32_t e

    .. cpp:member:: char32_t f

    .. cpp:member:: MyEnum g

    .. cpp:function:: static MyStruct new_()


    .. cpp:function:: uint8_t into_a()


.. cpp:class:: Opaque

    .. cpp:function:: static Opaque new_()


    .. cpp:function:: void assert_struct(MyStruct s) const

        See the `Rust documentation for something <https://docs.rs/Something/latest/struct.Something.html#method.something>`__ for more information.

        See the `Rust documentation for something_else <https://docs.rs/Something/latest/struct.Something.html#method.something_else>`__ for more information.

        Additional information: `1 <https://docs.rs/Something/latest/struct.Something.html#method.something_small>`__, `2 <https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something>`__


    .. cpp:function:: static size_t returns_usize()


    .. cpp:function:: static ImportedStruct returns_imported()


    .. cpp:function:: static int8_t cmp()

