``lifetimes::ffi``
==================

.. cpp:class:: Bar

.. cpp:class:: Foo

    .. cpp:function:: static Foo new_(const std::string_view x)

        Lifetimes: ``x`` must live at least as long as the output.

    .. cpp:function:: Bar get_bar() const

        Lifetimes: ``this`` must live at least as long as the output.
