#ifndef LLT_H
#define LLT_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct BasicBlock BasicBlock;

typedef struct Function Function;

typedef struct GlobalVar GlobalVar;

typedef struct Instruction Instruction;

typedef struct Module Module;

typedef struct ModuleHandle {
  struct Module *ptr;
} ModuleHandle;

typedef struct FuncHandle {
  struct Function *ptr;
} FuncHandle;

typedef struct BlockHandle {
  struct BasicBlock *ptr;
} BlockHandle;

typedef struct InstHandle {
  struct Instruction *ptr;
} InstHandle;

typedef struct GlobalVarHandle {
  struct GlobalVar *ptr;
} GlobalVarHandle;

int32_t add(int32_t a, int32_t b);

#endif  /* LLT_H */
