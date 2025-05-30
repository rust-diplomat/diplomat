#ifndef MutableCallbackHolder_H
#define MutableCallbackHolder_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "MutableCallbackHolder.d.h"





typedef struct DiplomatCallback_MutableCallbackHolder_new_func {
    const void* data;
    int32_t (*run_callback)(const void*, int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_MutableCallbackHolder_new_func;

MutableCallbackHolder* MutableCallbackHolder_new(DiplomatCallback_MutableCallbackHolder_new_func func_cb_wrap);

int32_t MutableCallbackHolder_call(MutableCallbackHolder* self, int32_t a);

void MutableCallbackHolder_destroy(MutableCallbackHolder* self);





#endif // MutableCallbackHolder_H
