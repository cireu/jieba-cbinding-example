#include <jieba_rustlib.h>
#include <stdio.h>

int main() {
  Jieba *handler;
  JiebaCStrVec out;
  uintptr_t *len;

  handler = jieba_make_handler();
  char *example = "我可以吞下玻璃而不伤身体";
  out = jieba_cut(handler, example);
  for (int i = 0; i < out.len; i = i+1) {
    printf("%s\n", out.ptr[i]);
  }

  jieba_destroy_cut_result(out);
  jieba_destroy_handler(handler);
}
