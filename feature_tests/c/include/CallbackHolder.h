#ifndef CallbackHolder_H
#define CallbackHolder_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "CallbackHolder.d.h"





typedef struct DiplomatCallback_CallbackHolder_new_func {
    const void* data;
    int32_t (*run_callback)(const void*, int32_t );
    void (*destructor)(const void*);
} DiplomatCallback_CallbackHolder_new_func;

CallbackHolder* CallbackHolder_new(DiplomatCallback_CallbackHolder_new_func func_cb_wrap);

int32_t CallbackHolder_call(const CallbackHolder* self, int32_t a);

void CallbackHolder_destroy(CallbackHolder* self);





#endif // CallbackHolder_H
