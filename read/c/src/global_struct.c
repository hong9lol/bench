#define NUM_ITERATIONS 1000000

typedef struct {
  int value;
} GlobalStruct;

GlobalStruct global_data;

void increment_global() {
  for (int i = 0; i < NUM_ITERATIONS; i++) {
    if (global_data.value % 2 == 0) {
      int a = 0;
    }
  }
}

void run() { increment_global(); }