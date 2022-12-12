``option::ffi``
===============

.. cpp:class:: OptionOpaque

    .. cpp:function:: static std::optional<OptionOpaque> new_(int32_t i)


    .. cpp:function:: static std::optional<OptionOpaque> new_none()


    .. cpp:function:: static OptionStruct new_struct()


    .. cpp:function:: static OptionStruct new_struct_nones()


    .. cpp:function:: void assert_integer(int32_t i) const


    .. cpp:function:: static bool option_opaque_argument(const OptionOpaque* arg)


.. cpp:class:: OptionOpaqueChar

    .. cpp:function:: void assert_char(char32_t ch) const


.. cpp:struct:: OptionStruct

    .. cpp:member:: std::optional<OptionOpaque> a

    .. cpp:member:: std::optional<OptionOpaqueChar> b

    .. cpp:member:: uint32_t c

    .. cpp:member:: std::optional<OptionOpaque> d
