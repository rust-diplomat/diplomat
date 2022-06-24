``lifetimes::ffi``
==================

.. js:class:: Bar

    .. js:function:: foo()

.. js:class:: Foo

    .. js:staticfunction:: new(x)

    .. js:function:: get_bar()

.. js:class:: One

    .. js:staticfunction:: transitivity(hold, nohold)

    .. js:staticfunction:: cycle(hold, nohold)

    .. js:staticfunction:: many_dependents(a, b, c, d, nohold)

    .. js:staticfunction:: return_outlives_param(hold, nohold)

    .. js:staticfunction:: diamond_top(top, left, right, bottom)

    .. js:staticfunction:: diamond_left(top, left, right, bottom)

    .. js:staticfunction:: diamond_right(top, left, right, bottom)

    .. js:staticfunction:: diamond_bottom(top, left, right, bottom)

    .. js:staticfunction:: diamond_and_nested_types(a, b, c, d, nohold)

.. js:class:: Two
