// Author: Y.P.
#ifndef IR
#define IR

#include "type.h"
#include "inst.h"
#include "llt/arena.h"


typedef struct IrInst {
    InstRKind kind;
    TypeKind type;

    ValueId a;
    ValueId b;
    ValueId c;

    int64_t imm;
} IrInst;

typedef struct IrBlock {
    InstId start;
    InstId count;

    BlockId *succ;
    uint32_t succ_count;

    BlockId *pred;
    uint32_t pred_count;
} IrBlock;

typedef struct IrFunction {
    IrInst *insts;
    size_t inst_count;

    IrBlock *blocks;
    size_t block_count;

    ValueId *params;
    size_t param_count;
} IrFunction;

typedef struct IrModule {
    IrFunction *funcs;
    size_t func_count;
} IrModule;

typedef struct IRContext {
    Arena *arena;

    IrModule *module;

    TypeKind *types;
    size_t type_count;

    ValueId next_value;
} IRContext;

#endif