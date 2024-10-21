#define NUM_ITERATIONS 1000000

int global_var = 0;

void increment_global() {
  for (int i = 0; i < NUM_ITERATIONS; i++) {
    if (global_var % 2 == 0) {
      int a = 0;
    }
  }
}

void run() { increment_global(); }
