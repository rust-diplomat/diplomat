``result::ffi``
===============

.. cpp:enum-struct:: ErrorEnum

    .. cpp:enumerator:: Foo

    .. cpp:enumerator:: Bar

.. cpp:struct:: ErrorStruct

    .. cpp:member:: int32_t i

    .. cpp:member:: int32_t j

.. cpp:class:: ResultOpaque

    .. cpp:function:: static diplomat::result<ResultOpaque, ErrorEnum> new_(int32_t i)


    .. cpp:function:: static diplomat::result<ResultOpaque, ErrorEnum> new_failing_foo()


    .. cpp:function:: static diplomat::result<ResultOpaque, ErrorEnum> new_failing_bar()


    .. cpp:function:: static diplomat::result<ResultOpaque, std::monostate> new_failing_unit()


    .. cpp:function:: static diplomat::result<ResultOpaque, ErrorStruct> new_failing_struct(int32_t i)


    .. cpp:function:: static diplomat::result<std::monostate, ResultOpaque> new_in_err(int32_t i)


    .. cpp:function:: static diplomat::result<ErrorEnum, ResultOpaque> new_in_enum_err(int32_t i)


    .. cpp:function:: void assert_integer(int32_t i) const

