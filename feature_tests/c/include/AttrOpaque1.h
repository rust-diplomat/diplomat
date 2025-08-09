#ifndef AttrOpaque1_H
#define AttrOpaque1_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AttrEnum.d.h"
#include "Unnamespaced.d.h"

#include "AttrOpaque1.d.h"





typedef struct DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result { bool is_ok;} DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result;

typedef struct DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t {
    const void* data;
    DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t_result (*run_callback)(const void*);
    void (*destructor)(const void*);
} DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t;

AttrOpaque1* namespace_AttrOpaque1_new(void);

void namespace_AttrOpaque1_test_namespaced_callback(DiplomatCallback_namespace_AttrOpaque1_test_namespaced_callback__t _t_cb_wrap);

int32_t namespace_AttrOpaque1_mac_test(void);

int32_t namespace_AttrOpaque1_hello(void);

uint8_t namespace_AttrOpaque1_method(const AttrOpaque1* self);

uint8_t renamed_on_abi_only(const AttrOpaque1* self);

void namespace_AttrOpaque1_use_unnamespaced(const AttrOpaque1* self, const Unnamespaced* _un);

void namespace_AttrOpaque1_use_namespaced(const AttrOpaque1* self, AttrEnum _n);

void namespace_AttrOpaque1_destroy(AttrOpaque1* self);





#endif // AttrOpaque1_H
