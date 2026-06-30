#ifndef LLT_H
#define LLT_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Module Module;

typedef struct ModuleHandle {
  struct Module *ptr;
} ModuleHandle;

int32_t add(int32_t a, int32_t b);

#endif  /* LLT_H */
