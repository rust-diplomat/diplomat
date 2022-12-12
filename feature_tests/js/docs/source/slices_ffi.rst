``slices::ffi``
===============

.. js:class:: Float64Vec

    .. js:staticfunction:: new(v)
        - Note: ``v`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


    .. js:function:: fill_slice(v)
        - Note: ``v`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


    .. js:function:: set_value(new_slice)
        - Note: ``new_slice`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


.. js:class:: MyString

    .. js:staticfunction:: new(v)

    .. js:function:: set_str(new_str)

    .. js:function:: get_str()
