``attrs::ffi``
==============

.. cpp:enum-struct:: AttrEnum

    .. cpp:enumerator:: A

    .. cpp:enumerator:: B

    .. cpp:enumerator:: C

.. cpp:class:: AttrOpaque1

    .. cpp:function:: static AttrOpaque1 new_()


    .. cpp:function:: uint8_t method() const


    .. cpp:function:: uint8_t abirenamed() const


    .. cpp:function:: void method_disabledcpp() const


    .. cpp:function:: void use_unnamespaced(const Unnamespaced& _un) const


    .. cpp:function:: void use_namespaced(AttrEnum _n) const


.. cpp:class:: AttrOpaque2

.. cpp:class:: Comparable

    .. cpp:function:: static Comparable new_(uint8_t int)


    .. cpp:function:: int8_t cmp(const Comparable& other) const


.. cpp:class:: MyIndexer

    .. cpp:function:: diplomat::result<const std::string_view, std::monostate> get(size_t i) const

        Lifetimes: ``this`` must live at least as long as the output.


.. cpp:class:: MyIterable

    .. cpp:function:: static MyIterable new_(const diplomat::span<const uint8_t> x)


    .. cpp:function:: MyIterator iter() const

        Lifetimes: ``this`` must live at least as long as the output.


.. cpp:class:: MyIterator

    .. cpp:function:: std::optional<uint8_t> next()


.. cpp:class:: OpaqueIterable

    .. cpp:function:: OpaqueIterator iter() const

        Lifetimes: ``this`` must live at least as long as the output.


.. cpp:class:: OpaqueIterator

    .. cpp:function:: std::optional<AttrOpaque1> next()


.. cpp:class:: Unnamespaced

    .. cpp:function:: static Unnamespaced make(AttrEnum _e)


    .. cpp:function:: void use_namespaced(const AttrOpaque1& _n) const

