``ownership::ffi``
==================

.. cpp:class:: CountedOpaque

    .. cpp:function:: static CountedOpaque new_(const Counter& counter)

.. cpp:class:: Counter

    Counts how many distinct ``CountedOpaque`` objects are instanciated

    .. cpp:function:: static Counter new_()

    .. cpp:function:: size_t count() const

.. cpp:class:: OwnershipEater

    "Ownership is a delicous dish." — OwnershipEater

    .. cpp:function:: static OwnershipEater new_()
