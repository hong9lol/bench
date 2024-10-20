#define NUM_ITERATIONS 1000000

int global_var = 0;

void increment_global() {
  for (int i = 0; i < NUM_ITERATIONS; i++) {
    global_var++;
  }
}

void run() { increment_global(); }