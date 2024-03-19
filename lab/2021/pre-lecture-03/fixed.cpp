#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

// There are at least 7 bugs relating to memory on this snippet.
// Find them all!

// Vec is short for "vector", a common term for a resizable array.
// For simplicity, our vector type can only hold ints.
typedef struct {
	int* data;     // Pointer to our array on the heap
	int  length;   // How many elements are in our array
	int  capacity; // How many elements our array can hold
} Vec;

/* Return a pointer to a new, empty Vec object. */
Vec* vec_new() {
	// 1
	Vec* vec = (Vec*)malloc(sizeof(Vec));
	vec->length = 0;
	// 2
	vec->capacity = 1;
	vec->data = (int*) malloc(vec->capacity * sizeof(int));
	return vec;
}

/* Push a new integer `n` onto `vec`. */
void vec_push(Vec* vec, int n) {
	if (vec->length == vec->capacity) {
		/* If previously-allocated space has been filled, 
			 allocate a new array for the vector contents 
			 with 2x the capacity, and copy over data. */
		int new_capacity = vec->capacity * 2;
		// 3
		int* new_data = (int*) malloc(new_capacity * sizeof(int));
		assert(new_data != NULL); /* Check that `malloc` succeeded. */

		for (int i = 0; i < vec->length; ++i) {
			new_data[i] = vec->data[i];
		}
		// 4
		free(vec->data);

		vec->data = new_data;
		vec->capacity = new_capacity;
	}

	vec->data[vec->length] = n;
	++vec->length;
}

/* Free the vector and associated data. */
void vec_free(Vec* vec) {
	// 5
	free(vec->data);
	free(vec);
}

/* Test program. */
int main() {
	Vec* vec = vec_new();
	vec_push(vec, 107);
	// 6
	int n = vec->data[0];
	vec_push(vec, 110);
	printf("%d\n", n);

	// 7
	vec_free(vec);

	return 0;
}