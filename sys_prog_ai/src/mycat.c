#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

int main(int argc, char **argv) {
    if (argc < 2) {
        write(2, "usage: mycat <file>\n", 21);
        return 1;
    }

    int fd = open(argv[1], O_RDONLY);
    if (fd < 0) {
        write(2, "open failed\n", 13);
        return 1;
    }

    char buf[1024];
    ssize_t n;
    while ((n = read(fd, buf, sizeof buf)) > 0) {
        if (write(1, buf, n) != n) {
            perror("write");
            close(fd);
            return 1;
        }
    }

    close(fd);
    return 0;
}

// gcc -Wall -Wextra -o mycat mycat.c
// strace -o trace.log ./mycat /etc/hostname   # или любой существующий текстовый файл
// cat trace.log   # посмотри, что получилось