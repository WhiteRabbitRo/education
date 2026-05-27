#include <sys/stat.h>
#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
    if (argc < 2) {
        write(2, "usage: mystat <file>\n", 21);
        return 1;
    }

    struct stat st;
    if (stat(argv[1], &st) < 0) {
        perror("stat");
        return 1;
    }

    printf("File: %s\n", argv[1]);
    printf("Size: %ld bytes\n", st.st_size);
    printf("Type: %s\n", S_ISDIR(st.st_mode) ? "directory" : (S_ISLNK(st.st_mode) ? "link" : "regular file"));
    printf("Permissions: %04o\n", st.st_mode & 07777);

    return 0;
}

// gcc -Wall -Wextra -o mystat mystat.c
// strace -o trace.log -e trace=stat ./mystat /etc/hostname