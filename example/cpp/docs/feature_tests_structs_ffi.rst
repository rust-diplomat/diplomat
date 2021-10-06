``feature_tests::structs::ffi``
===============================

.. cpp:struct:: MyStruct

    .. cpp:member:: uint8_t a

    .. cpp:member:: bool b

    .. cpp:member:: uint8_t c

    .. cpp:member:: uint64_t d

    .. cpp:member:: int32_t e

    .. cpp:member:: char32_t f

    .. cpp:function:: static MyStruct new_()

.. cpp:class:: Opaque

    .. cpp:function:: static Opaque new_()

    .. cpp:function:: void assert_struct(MyStruct s) const
