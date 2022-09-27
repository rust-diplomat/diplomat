#ifndef RefList_HPP
#define RefList_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "RefList.h"

class RefList;

/**
 * A destruction policy for using RefList with std::unique_ptr.
 */
struct RefListDeleter {
  void operator()(capi::RefList* l) const noexcept {
    capi::RefList_destroy(l);
  }
};
class RefList {
 public:

  /**
   * Lifetimes: `data` must live at least as long as the output.
   */
  static RefList node(const int32_t& data);
  inline const capi::RefList* AsFFI() const { return this->inner.get(); }
  inline capi::RefList* AsFFIMut() { return this->inner.get(); }
  inline RefList(capi::RefList* i) : inner(i) {}
  RefList() = default;
  RefList(RefList&&) noexcept = default;
  RefList& operator=(RefList&& other) noexcept = default;
 private:
  std::unique_ptr<capi::RefList, RefListDeleter> inner;
};


inline RefList RefList::node(const int32_t& data) {
  return RefList(capi::RefList_node(data));
}
#endif
