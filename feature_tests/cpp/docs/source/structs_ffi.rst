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


    .. cpp:function:: static MyEnum get_a()


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


    .. cpp:function:: static std::optional<Opaque> try_from_utf8(const std::string_view input)


    .. cpp:function:: static Opaque from_str(const std::string_view input)

        Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).



    .. cpp:function:: template<typename W> void get_debug_str_to_write(W& write) const


    .. cpp:function:: std::string get_debug_str() const


    .. cpp:function:: void assert_struct(MyStruct s) const

        See the `Rust documentation for something <https://docs.rs/Something/latest/struct.Something.html#method.something>`__ for more information.

        See the `Rust documentation for something_else <https://docs.rs/Something/latest/struct.Something.html#method.something_else>`__ for more information.

        Additional information: `1 <https://docs.rs/Something/latest/struct.Something.html#method.something_small>`__, `2 <https://docs.rs/SomethingElse/latest/struct.SomethingElse.html#method.something>`__


    .. cpp:function:: static size_t returns_usize()


    .. cpp:function:: static ImportedStruct returns_imported()


    .. cpp:function:: static int8_t cmp()


.. cpp:class:: OpaqueMutexedString

    .. cpp:function:: static OpaqueMutexedString from_usize(size_t number)


    .. cpp:function:: void change(size_t number) const


    .. cpp:function:: size_t get_len_and_add(size_t other) const


    .. cpp:function:: const std::string_view dummy_str() const

        Lifetimes: ``this`` must live at least as long as the output.


    .. cpp:function:: Utf16Wrap wrapper() const


.. cpp:class:: Utf16Wrap

    .. cpp:function:: static Utf16Wrap from_utf16(const std::u16string_view input)



    .. cpp:function:: template<typename W> void get_debug_str_to_write(W& write) const


    .. cpp:function:: std::string get_debug_str() const


    .. cpp:function:: const std::u16string_view borrow_cont() const

        Lifetimes: ``this`` must live at least as long as the output.


    .. cpp:function:: const std::u16string_view owned() const

