``slices::ffi``
===============

.. cpp:class:: Float64Vec

    .. cpp:function:: static Float64Vec new_(const diplomat::span<const double> v)


    .. cpp:function:: static Float64Vec new_bool(const diplomat::span<const bool> v)


    .. cpp:function:: static Float64Vec new_i16(const diplomat::span<const int16_t> v)


    .. cpp:function:: static Float64Vec new_u16(const diplomat::span<const uint16_t> v)


    .. cpp:function:: static Float64Vec new_isize(const diplomat::span<const intptr_t> v)


    .. cpp:function:: static Float64Vec new_usize(const diplomat::span<const size_t> v)


    .. cpp:function:: static Float64Vec new_f64_be_bytes(const diplomat::span<const uint8_t> v)


    .. cpp:function:: static Float64Vec new_from_owned(const diplomat::span<double> v)


    .. cpp:function:: const diplomat::span<double> as_boxed_slice() const


    .. cpp:function:: const diplomat::span<const double> as_slice() const

        Lifetimes: ``this`` must live at least as long as the output.


    .. cpp:function:: void fill_slice(const diplomat::span<double> v) const


    .. cpp:function:: void set_value(const diplomat::span<const double> new_slice)



    .. cpp:function:: template<typename W> void to_string_to_writeable(W& w) const


    .. cpp:function:: std::string to_string() const


    .. cpp:function:: const diplomat::span<const double> borrow() const

        Lifetimes: ``this`` must live at least as long as the output.


.. cpp:class:: MyString

    .. cpp:function:: static MyString new_(const std::string_view v)


    .. cpp:function:: static MyString new_unsafe(const std::string_view v)

        Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).


    .. cpp:function:: static MyString new_owned(const std::string_view v)


    .. cpp:function:: void set_str(const std::string_view new_str)



    .. cpp:function:: template<typename W> void get_str_to_writeable(W& writeable) const


    .. cpp:function:: std::string get_str() const

