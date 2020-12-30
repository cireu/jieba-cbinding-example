#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Jieba segmentation
 */
typedef struct Jieba Jieba;

typedef struct {
  char **ptr;
  uintptr_t len;
  uintptr_t cap;
} JiebaCStrVec;

JiebaCStrVec jieba_cut(Jieba *handler, char *string);

void jieba_destroy_cut_result(JiebaCStrVec cut_res);

void jieba_destroy_handler(Jieba *handler);

Jieba *jieba_make_handler(void);
