#ifndef RefList_D_HPP
#define RefList_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "RefList.d.h"


class RefListParameter;


class RefList {
public:
	static std::unique_ptr<RefList> inline node(const RefListParameter& data);

	inline capi::RefList* AsFFI();

	inline ~RefList();

private:
	RefList() = delete;
};





#endif // RefList_D_HPP
