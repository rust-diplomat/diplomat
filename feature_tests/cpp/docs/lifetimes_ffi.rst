``lifetimes::ffi``
==================

.. cpp:class:: Bar

.. cpp:class:: Foo

    .. cpp:function:: static Foo new_(const std::string_view x)

        Lifetimes: ``x`` must live at least as long as the output.

    .. cpp:function:: Bar get_bar() const

        Lifetimes: ``this`` must live at least as long as the output.

.. cpp:class:: One

    .. cpp:function:: static One transitivity(const One& hold, const One& nohold)

        Lifetimes: ``hold`` must live at least as long as the output.

    .. cpp:function:: static One cycle(const Two& hold, const One& nohold)

        Lifetimes: ``hold`` must live at least as long as the output.

    .. cpp:function:: static One many_dependents(const One& a, const One& b, const Two& c, const Two& d, const Two& nohold)

        Lifetimes: ``a``, ``b``, ``c``, ``d`` must live at least as long as the output.

    .. cpp:function:: static One return_outlives_param(const Two& hold, const One& nohold)

        Lifetimes: ``hold`` must live at least as long as the output.

    .. cpp:function:: static One diamond_top(const One& top, const One& left, const One& right, const One& bottom)

        Lifetimes: ``top``, ``left``, ``right``, ``bottom`` must live at least as long as the output.

    .. cpp:function:: static One diamond_left(const One& top, const One& left, const One& right, const One& bottom)

        Lifetimes: ``left``, ``bottom`` must live at least as long as the output.

    .. cpp:function:: static One diamond_right(const One& top, const One& left, const One& right, const One& bottom)

        Lifetimes: ``right``, ``bottom`` must live at least as long as the output.

    .. cpp:function:: static One diamond_bottom(const One& top, const One& left, const One& right, const One& bottom)

        Lifetimes: ``bottom`` must live at least as long as the output.

    .. cpp:function:: static One diamond_and_nested_types(const One& a, const One& b, const One& c, const One& d, const One& nohold)

        Lifetimes: ``a``, ``b``, ``c``, ``d`` must live at least as long as the output.

.. cpp:class:: Two
