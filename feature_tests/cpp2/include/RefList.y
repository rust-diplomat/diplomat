#ifndef RefList_H
#define RefList_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


class RefListParameter;


class RefList {
public:
	static std::unique_ptr<RefList> node(const RefListParameter& data);

	inline capi::RefList AsFFI() {
		return reinterpret_cast::<capi::RefList>(this);
	}

	~RefList() {
		RefList_destroy(AsFFI());
	}

private:
	RefList() = delete;
}





#endif // RefList_HPP
