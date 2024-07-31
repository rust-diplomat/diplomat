#ifndef Diplomat_callback_structs_H
#define Diplomat_callback_structs_H

typedef struct DiplomatCallback_void {
	const void* data;
	void (*run_callback)(const void*, ...);
	void (*destructor)(const void*);
} DiplomatCallback_void;

typedef struct DiplomatCallback_int32_t {
	const void* data;
	int32_t (*run_callback)(const void*, ...);
	void (*destructor)(const void*);
} DiplomatCallback_int32_t;


#endif
