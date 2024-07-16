#ifndef Unnamespaced_H
#define Unnamespaced_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AttrEnum.d.h"
#include "AttrOpaque1.d.h"

#include "Unnamespaced.d.h"






Unnamespaced* Unnamespaced_make(AttrEnum _e) {
    Unnamespaced* namespace_Unnamespaced_make(AttrEnum _e);
    return namespace_Unnamespaced_make(_e);
}

void Unnamespaced_use_namespaced(const Unnamespaced* self, const AttrOpaque1* _n) {
    void namespace_Unnamespaced_use_namespaced(const Unnamespaced* self, const AttrOpaque1* _n);
    return namespace_Unnamespaced_use_namespaced(self, _n);
}


void namespace_Unnamespaced_destroy(Unnamespaced* self);





#endif // Unnamespaced_H
