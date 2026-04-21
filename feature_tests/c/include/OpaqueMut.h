#ifndef OpaqueMut_H
#define OpaqueMut_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "OpaqueMut.d.h"






OpaqueMut* OpaqueMut_new(void);

void OpaqueMut_destroy(OpaqueMut* self);





#endif // OpaqueMut_H
