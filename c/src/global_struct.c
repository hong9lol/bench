#define NUM_ITERATIONS 1000000

typedef struct {
  int value;
} GlobalStruct;

GlobalStruct global_data;

void increment_global() {
  for (int i = 0; i < NUM_ITERATIONS; i++) {
    global_data.value++;
  }
}

void run() { increment_global(); }