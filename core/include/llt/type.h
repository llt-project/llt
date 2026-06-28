// Author: Y.P.
#ifndef TYPE
#define TYPE 


typedef enum {
    TY_I16,
    TY_I32,
    TY_I64,
    TY_F32,
    TY_F64,
    TY_VOID,
    TY_PTR
} TypeKind;

typedef uint32_t ValueId;
typedef uint32_t InstId;
typedef uint32_t BlockId;
typedef uint32_t FuncId;

typedef struct symbol  Symbol;

struct symbol 
{
    const char *name;
    void *definition;
};


#endif