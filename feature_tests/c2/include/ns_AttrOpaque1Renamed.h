#ifndef ns_AttrOpaque1Renamed_H
#define ns_AttrOpaque1Renamed_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Unnamespaced.d.h"
#include "ns_RenamedAttrEnum.d.h"

#include "ns_AttrOpaque1Renamed.d.h"






ns_AttrOpaque1Renamed* ns_AttrOpaque1Renamed_new() {
    ns_AttrOpaque1Renamed* namespace_AttrOpaque1_new();
    return namespace_AttrOpaque1_new();
}

uint8_t ns_AttrOpaque1Renamed_method(const ns_AttrOpaque1Renamed* self) {
    uint8_t namespace_AttrOpaque1_method(const ns_AttrOpaque1Renamed* self);
    return namespace_AttrOpaque1_method(self);
}

uint8_t ns_AttrOpaque1Renamed_abirenamed(const ns_AttrOpaque1Renamed* self) {
    uint8_t renamed_on_abi_only(const ns_AttrOpaque1Renamed* self);
    return renamed_on_abi_only(self);
}

void ns_AttrOpaque1Renamed_use_unnamespaced(const ns_AttrOpaque1Renamed* self, const Unnamespaced* _un) {
    void namespace_AttrOpaque1_use_unnamespaced(const ns_AttrOpaque1Renamed* self, const Unnamespaced* _un);
    return namespace_AttrOpaque1_use_unnamespaced(self, _un);
}

void ns_AttrOpaque1Renamed_use_namespaced(const ns_AttrOpaque1Renamed* self, ns_RenamedAttrEnum _n) {
    void namespace_AttrOpaque1_use_namespaced(const ns_AttrOpaque1Renamed* self, ns_RenamedAttrEnum _n);
    return namespace_AttrOpaque1_use_namespaced(self, _n);
}


void ns_AttrOpaque1Renamed_destroy(ns_AttrOpaque1Renamed* self) {
    void namespace_AttrOpaque1_destroy(ns_AttrOpaque1Renamed* self);
    namespace_AttrOpaque1_destroy(self);
}





#endif // ns_AttrOpaque1Renamed_H
