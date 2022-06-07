``lifetimes::ffi``
==================

.. cpp:class:: Bar

.. cpp:class:: Foo

    .. cpp:function:: static Foo new_(const std::string_view x)

    .. cpp:function:: Bar get_bar() const

.. cpp:class:: One

    .. cpp:function:: static One transitivity(const One& hold, const One& nohold)

    .. cpp:function:: static One cycle(const Two& hold, const One& nohold)

    .. cpp:function:: static One many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold)

    .. cpp:function:: static One return_outlives_param(const Two& hold, const One& nohold)

    .. cpp:function:: static One diamond_top(const One& top, const One& left, const One& right, const One& bottom)

    .. cpp:function:: static One diamond_left(const One& top, const One& left, const One& right, const One& bottom)

    .. cpp:function:: static One diamond_right(const One& top, const One& left, const One& right, const One& bottom)

    .. cpp:function:: static One diamond_bottom(const One& top, const One& left, const One& right, const One& bottom)

    .. cpp:function:: static One diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold)

.. cpp:class:: Two
