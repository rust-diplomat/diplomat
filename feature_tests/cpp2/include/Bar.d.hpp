#ifndef Bar_D_HPP
#define Bar_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "Bar.d.h"




class Bar {
public:

  inline const capi::Bar* AsFFI() const;
  inline capi::Bar* AsFFI();
  inline ~Bar();
private:
  Bar() = delete;
};





#endif // Bar_D_HPP
