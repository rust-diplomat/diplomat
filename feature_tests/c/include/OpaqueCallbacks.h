#ifndef OpaqueCallbacks_H
#define OpaqueCallbacks_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "MyString.d.h"

#include "OpaqueCallbacks.d.h"





typedef struct DiplomatCallback_OpaqueCallbacks_ret_op_f {
    const void* data;
    const MyString* (*run_callback)(const void*, const MyString* );
    void (*destructor)(const void*);
} DiplomatCallback_OpaqueCallbacks_ret_op_f;
typedef struct DiplomatCallback_OpaqueCallbacks_ctor_f {
    const void* data;
    const MyString* (*run_callback)(const void*, const MyString* );
    void (*destructor)(const void*);
} DiplomatCallback_OpaqueCallbacks_ctor_f;
typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_self_cb {
    const void* data;
    const MyString* (*run_callback)(const void*, const MyString* );
    void (*destructor)(const void*);
} DiplomatCallback_OpaqueCallbacks_opaque_cb_self_cb;
typedef struct DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_cb {
    const void* data;
    const MyString* (*run_callback)(const void*, const MyString* );
    void (*destructor)(const void*);
} DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_cb;

const MyString* OpaqueCallbacks_ret_op(DiplomatCallback_OpaqueCallbacks_ret_op_f f_cb_wrap, const MyString* st);

OpaqueCallbacks* OpaqueCallbacks_ctor(DiplomatCallback_OpaqueCallbacks_ctor_f f_cb_wrap, const MyString* st);

const MyString* OpaqueCallbacks_opaque_cb_self(const OpaqueCallbacks* self, DiplomatCallback_OpaqueCallbacks_opaque_cb_self_cb cb_cb_wrap, const MyString* st);

const MyString* OpaqueCallbacks_opaque_cb_mut_self(OpaqueCallbacks* self, DiplomatCallback_OpaqueCallbacks_opaque_cb_mut_self_cb cb_cb_wrap, const MyString* st);

void OpaqueCallbacks_destroy(OpaqueCallbacks* self);





#endif // OpaqueCallbacks_H
