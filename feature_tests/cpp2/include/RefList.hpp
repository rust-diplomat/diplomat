#ifndef RefList_HPP
#define RefList_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




	static std::unique_ptr<RefList> inline node(const RefListParameter& data) {
	// TODO
}
inline capi::RefList* AsFFI() {
	return reinterpret_cast::<capi::RefList>(this);
}
inline ~RefList() {
	capi::RefList_destroy(AsFFI());
}


#endif // RefList_HPP
