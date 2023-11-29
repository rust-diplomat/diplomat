``lifetimes::ffi``
==================

.. cpp:class:: Bar

.. cpp:struct:: BorrowedFields

    .. cpp:member:: std::u16string_view a

    .. cpp:member:: std::string_view b

    .. cpp:member:: std::string_view c
        Warning: Setting ill-formed UTF-8 is undefined behavior (and may be memory-unsafe).

.. cpp:struct:: BorrowedFieldsReturning

    .. cpp:member:: std::string_view bytes

.. cpp:class:: Foo

    .. cpp:function:: static Foo new_(const std::string_view x)

        Lifetimes: ``x`` must live at least as long as the output.


    .. cpp:function:: Bar get_bar() const

        Lifetimes: ``this`` must live at least as long as the output.


    .. cpp:function:: static Foo new_static(const std::string_view x)

        Lifetimes: ``x`` must live for the duration of the program.


    .. cpp:function:: BorrowedFieldsReturning as_returning() const

        Lifetimes: ``this`` must live at least as long as the output.


    .. cpp:function:: static Foo extract_from_fields(BorrowedFields fields)

        Lifetimes: ``fields`` must live at least as long as the output.


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


    .. cpp:function:: static One implicit_bounds(const One& explicit_hold, const One& implicit_hold, const One& nohold)

        Lifetimes: ``explicit_hold``, ``implicit_hold`` must live at least as long as the output.


    .. cpp:function:: static One implicit_bounds_deep(const One& explicit_, const One& implicit_1, const One& implicit_2, const One& nohold)

        Lifetimes: ``explicit_``, ``implicit_1``, ``implicit_2`` must live at least as long as the output.


.. cpp:class:: Two
