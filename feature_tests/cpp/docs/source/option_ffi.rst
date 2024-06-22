``option::ffi``
===============

.. cpp:class:: OptionOpaque

    .. cpp:function:: static std::optional<OptionOpaque> new_(int32_t i)


    .. cpp:function:: static std::optional<OptionOpaque> new_none()


    .. cpp:function:: static std::optional<OptionStruct> returns()


    .. cpp:function:: std::optional<intptr_t> option_isize() const


    .. cpp:function:: std::optional<size_t> option_usize() const


    .. cpp:function:: std::optional<int32_t> option_i32() const


    .. cpp:function:: std::optional<uint32_t> option_u32() const


    .. cpp:function:: static OptionStruct new_struct()


    .. cpp:function:: static OptionStruct new_struct_nones()


    .. cpp:function:: void assert_integer(int32_t i) const


    .. cpp:function:: static bool option_opaque_argument(const OptionOpaque* arg)


.. cpp:class:: OptionOpaqueChar

    .. cpp:function:: void assert_char(char32_t ch) const


.. cpp:class:: OptionString

    .. cpp:function:: static std::optional<OptionString> new_(const std::string_view diplomat_str)



    .. cpp:function:: template<typename W> diplomat::result<std::monostate, std::monostate> write_to_write(W& write) const


    .. cpp:function:: diplomat::result<std::string, std::monostate> write() const


    .. cpp:function:: std::optional<const std::string_view> borrow() const

        Lifetimes: ``this`` must live at least as long as the output.


.. cpp:struct:: OptionStruct

    .. cpp:member:: std::optional<OptionOpaque> a

    .. cpp:member:: std::optional<OptionOpaqueChar> b

    .. cpp:member:: uint32_t c

    .. cpp:member:: std::optional<OptionOpaque> d
