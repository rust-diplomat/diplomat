``structs::ffi``
================

.. cpp:struct:: Alpha

    .. cpp:member:: uint32_t x

    .. cpp:member:: uint32_t y

.. cpp:struct:: Beta

    .. cpp:member:: Alpha alpha_field

    .. cpp:function:: static Beta new_(uint32_t x, uint32_t y)

.. cpp:struct:: MyStruct

    .. cpp:member:: uint8_t a

    .. cpp:member:: bool b

    .. cpp:member:: uint8_t c

    .. cpp:member:: uint64_t d

    .. cpp:member:: int32_t e

    .. cpp:member:: char32_t f

    .. cpp:function:: static MyStruct new_()

    .. cpp:function:: void consume()

.. cpp:class:: Opaque

    .. cpp:function:: static Opaque new_()

    .. cpp:function:: void assert_struct(MyStruct s) const
