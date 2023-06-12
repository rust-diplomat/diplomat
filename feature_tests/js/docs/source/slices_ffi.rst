``slices::ffi``
===============

.. js:class:: Float64Vec

    .. js:function:: new(v)
        - Note: ``v`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


    .. js:method:: fill_slice(v)
        - Note: ``v`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


    .. js:method:: set_value(new_slice)
        - Note: ``new_slice`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


.. js:class:: MyString

    .. js:function:: new(v)

    .. js:method:: set_str(new_str)

    .. js:method:: get_str()
