#ifndef RefList_D_HPP
#define RefList_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.h"
#include "RefList.d.h"


class RefListParameter;


class RefList {
public:

  inline static std::unique_ptr<RefList> node(const RefListParameter& data);

  inline const capi::RefList* AsFFI() const;
  inline capi::RefList* AsFFI();
  inline ~RefList();
private:
  RefList() = delete;
};





#endif // RefList_D_HPP
