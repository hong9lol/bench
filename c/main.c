#include <stdio.h>
#include <time.h>

void run();
#define TEST_NUM 1
int main() {
  clock_t start, end;
  double cpu_time_used = 0.0;
  for (int i = 0; i < TEST_NUM; i++) {
    start = clock();
    run();
    end = clock();

    cpu_time_used += ((double)(end - start)) / CLOCKS_PER_SEC;
  }
  printf("%fms\n", (cpu_time_used * 1000) / TEST_NUM);

  return 0;
}