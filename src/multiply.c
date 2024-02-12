#include <stdio.h>
#include <stdint.h>

char* saved_ptr = NULL;

void ops(char* ptr) {
   if (saved_ptr != NULL) {
        printf("[C] This was: %p \n", ((void *) saved_ptr));
   } else {
    printf("[C] This is: %p \n", ((void *) ptr));
   }


    saved_ptr = ptr;

    return;
}
