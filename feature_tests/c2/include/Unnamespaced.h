#ifndef Unnamespaced_H
#define Unnamespaced_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ns_AttrOpaque1Renamed.d.h"
#include "ns_RenamedAttrEnum.d.h"

#include "Unnamespaced.d.h"






Unnamespaced* Unnamespaced_make(ns_RenamedAttrEnum _e) {
    Unnamespaced* namespace_Unnamespaced_make(ns_RenamedAttrEnum _e);
    return namespace_Unnamespaced_make(_e);
}

void Unnamespaced_use_namespaced(const Unnamespaced* self, const ns_AttrOpaque1Renamed* _n) {
    void namespace_Unnamespaced_use_namespaced(const Unnamespaced* self, const ns_AttrOpaque1Renamed* _n);
    return namespace_Unnamespaced_use_namespaced(self, _n);
}


void Unnamespaced_destroy(Unnamespaced* self) {
    void namespace_Unnamespaced_destroy(Unnamespaced* self);
    namespace_Unnamespaced_destroy(self);
}





#endif // Unnamespaced_H
