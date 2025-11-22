#include "uthash.h"
#include <string.h>
#include <stdlib.h>

// 1. The Data Structure (A "User")
struct my_struct {
    int id;                    /* The Key */
    char name[10];             /* The Value */
    UT_hash_handle hh;         /* The Magic: makes this structure hashable */
};

// 2. The Hash Table Head (Global pointer)
struct my_struct *users = NULL;

// 3. Helper: Add a user
void add_user(int user_id, const char *name) {
    struct my_struct *s;

    // Use macro to check if user exists
    HASH_FIND_INT(users, &user_id, s);
    if (s == NULL) {
        // Allocate memory (malloc)
        s = (struct my_struct *)malloc(sizeof(struct my_struct));
        s->id = user_id;
        // Use macro to Add
        HASH_ADD_INT(users, id, s);
    }
    strcpy(s->name, name);
}

// 4. Helper: Find a user
struct my_struct *find_user(int user_id) {
    struct my_struct *s;
    // Use macro to Find
    HASH_FIND_INT(users, &user_id, s);
    return s;
}

// 5. Helper: Delete all (Cleanup memory)
void delete_all() {
    struct my_struct *current_user, *tmp;
    // Use macro to Iterate safe against deletion
    HASH_ITER(hh, users, current_user, tmp) {
        HASH_DEL(users, current_user);
        free(current_user);
    }
}