#include <dlfcn.h>
#include <string.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdarg.h>
#include <errno.h>

#include <nsod_rust.h>

#define OPEN_SYMBOL "open"

int __hook_call_libc_open (const char * path, int flags, int mode) {

    dlerror(); // Clear any existing errors.

    int (*libc_open)(const char *, int, ...) = dlsym(RTLD_NEXT, OPEN_SYMBOL); // Find the next loaded 'open' function after this one.

    char * e = dlerror(); // e will be NULL if no error occured, or a string describing the error if one did occur.
    if (e != NULL) { 
        puts(e);
        exit(1);
    }

    if (libc_open == NULL) { // Theoretically dlsym could return NULL without error as symbols can have address zero, we wouldn't want to actually call the function if that was the case though.
        exit(1); 
    }

    return (libc_open(path, flags, mode));
}

int open(const char * path, int flags, ...) {
    // If needed, get the optional 'mode' argument
    int mode;
    if (__OPEN_NEEDS_MODE (flags))
    {
        va_list arg;
        va_start(arg, flags);
        mode = va_arg(arg, int);
        va_end(arg);
    }


    if (flags != O_RDONLY) { // Only hook open if in readonly mode, as we have no way to handle creating/writing to an actual file.
        return __hook_call_libc_open(path, flags, mode);
    }


    // Prepare pipe. It's worth doing this before we know if we're actually doing the hook to avoid having to parse cfg twice.
    int pipe_fds[2];
    if (pipe(pipe_fds) != 0) {
        return(-1); // failing to open a pipe should be handled the same as failing to open any other file.
    }
    int pipe_write = pipe_fds[1];
    int pipe_read = pipe_fds[0];
        
    
    // call rust function here:
    int rust_result = __nsod_rust_request(path, pipe_write);
    close(pipe_write);

    // Handle results from rust function call:
    if (rust_result != 0) { // abnormal result: fall back to libc open
        close(pipe_read);
        return __hook_call_libc_open(path, flags, mode);
    }

    return(pipe_read); // normal result: return fd with secret
}