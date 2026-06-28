// Author: Y.P.
#ifndef INST
#define INST


typedef enum {
    IR_CONST,

    IR_ADD,
    IR_SUB,
    IR_MUL,
    IR_DIV,

    IR_CMP,

    IR_LOAD,
    IR_STORE,

    IR_BR,
    IR_BR_COND,

    IR_PHI,

    IR_CALL,
    IR_RET
} InstRKind;

#endif