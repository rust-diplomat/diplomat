``structs::ffi``
================

.. cpp:struct:: MyStruct

    .. cpp:member:: uint8_t a

    .. cpp:member:: bool b

    .. cpp:member:: uint8_t c

    .. cpp:member:: uint64_t d

    .. cpp:member:: int32_t e

    .. cpp:member:: char32_t f

    .. cpp:member:: const std::string_view g

    .. cpp:function:: static MyStruct new_(const std::string_view s)

        Lifetimes: ``s`` must live at least as long as the output.

.. cpp:class:: Opaque

    .. cpp:function:: static Opaque new_()

    .. cpp:function:: void assert_struct(MyStruct s) const
