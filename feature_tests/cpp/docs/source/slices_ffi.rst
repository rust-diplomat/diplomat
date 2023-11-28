``slices::ffi``
===============

.. cpp:class:: Float64Vec

    .. cpp:function:: static Float64Vec new_(const diplomat::span<const double> v)


    .. cpp:function:: void fill_slice(diplomat::span<const double> v) const


    .. cpp:function:: void set_value(const diplomat::span<const double> new_slice)


.. cpp:class:: MyString

    .. cpp:function:: static MyString new_(const std::string_view v)


    .. cpp:function:: static MyString new_unsafe(const std::string_view v)

        Warning: Passing ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).


    .. cpp:function:: void set_str(const std::string_view new_str)


    .. cpp:function:: template<typename W> void get_str_to_writeable(W& writeable) const


    .. cpp:function:: std::string get_str() const

