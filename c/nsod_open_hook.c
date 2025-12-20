#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdio.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdarg.h>
#include <errno.h>

#include <nsod_rust.h>

#define OPEN_SYMBOL "open"
#define FOPEN_SYMBOL "fopen"
#define OPEN64_SYMBOL "open64"

// Call next loaded function with the symbol "open" (usually from libc).
// Usually called when we decide not to intercept the original open call.
int __nsod_fallback_open(const char * path, int flags, int mode) {

    dlerror(); // Clear any existing errors.

    int (*libc_open)(const char *, int, ...) = dlsym(RTLD_NEXT, OPEN_SYMBOL); // Find the next loaded 'open' function after this one.

    char * e = dlerror(); // e will be NULL if no error occured, or a string describing the error if one did occur.
    if (e != NULL) { 
        puts(e);
        exit(1);
    }

    /*
    Theoretically dlsym could return NULL without error as symbols can have address zero.
    We wouldn't want to actually call the function if that was the case though.
    */
    if (libc_open == NULL) {
        puts("NSOD: open symbol with address NULL.");
        exit(1); 
    }

    return (libc_open(path, flags, mode));
}

int open(const char * path, int flags, ...) {
    // If needed, get the optional 'mode' argument
    int mode;
    if (__OPEN_NEEDS_MODE (flags)) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, int);
        va_end(args);
    }

    // Only hook open if in readonly mode, as we have no way to handle creating/writing to an actual file.
    if (flags != O_RDONLY) { 
        return __nsod_fallback_open(path, flags, mode);
    }


    // It's worth doing this before we know if we're actually doing the hook to avoid having to parse cfg twice.
    int pipe_fds[2];
    if (pipe(pipe_fds) != 0) {
        return(-1); // Non-normal return. Errno will be set by the pipe() function.
    }
    int pipe_write = pipe_fds[1];
    int pipe_read = pipe_fds[0];
        
    int rust_result = __nsod_rust_request(path, pipe_write);

    // Handle results from rust function call:
    if (rust_result != 0) { // Non-normal return.
        close(pipe_read);
        return __nsod_fallback_open(path, flags, mode);
    }

    return(pipe_read); // Normal return: return fd with secret.
}

// =============================== OPEN64 =======================================

int __nsod_fallback_open64(const char * path, int flags, int mode) {

    dlerror(); // Clear any existing errors.

    int (*libc_open)(const char *, int, ...) = dlsym(RTLD_NEXT, OPEN64_SYMBOL); // Find the next loaded 'open' function after this one.

    char * e = dlerror(); // e will be NULL if no error occured, or a string describing the error if one did occur.
    if (e != NULL) { 
        puts(e);
        exit(1);
    }

    /*
    Theoretically dlsym could return NULL without error as symbols can have address zero.
    We wouldn't want to actually call the function if that was the case though.
    */
    if (libc_open == NULL) {
        puts("NSOD: open symbol with address NULL.");
        exit(1); 
    }

    return (libc_open(path, flags, mode));
}

int open64(const char * path, int flags, ...) {
    // If needed, get the optional 'mode' argument
    int mode;
    if (__OPEN_NEEDS_MODE (flags)) {
        va_list args;
        va_start(args, flags);
        mode = va_arg(args, int);
        va_end(args);
    }

    // Only hook open if in readonly mode, as we have no way to handle creating/writing to an actual file.
    if (flags != O_RDONLY) { 
        return __nsod_fallback_open64(path, flags, mode);
    }


    // It's worth doing this before we know if we're actually doing the hook to avoid having to parse cfg twice.
    int pipe_fds[2];
    if (pipe(pipe_fds) != 0) {
        return(-1); // Non-normal return. Errno will be set by the pipe() function.
    }
    int pipe_write = pipe_fds[1];
    int pipe_read = pipe_fds[0];
        
    int rust_result = __nsod_rust_request(path, pipe_write);

    // Handle results from rust function call:
    if (rust_result != 0) { // Non-normal return.
        close(pipe_read);
        return __nsod_fallback_open64(path, flags, mode);
    }

    return(pipe_read); // Normal return: return fd with secret.
}



// =============================== FOPEN =======================================

// Call next loaded function with the symbol "fopen" (usually from libc).
// Usually called when we decide not to intercept the original fopen call.
FILE * __nsod_fallback_fopen(const char * path, const char * mode) {

    dlerror(); // Clear any existing errors.

    FILE * (*libc_fopen)(const char *, const char *) = dlsym(RTLD_NEXT, FOPEN_SYMBOL); // Find the next loaded 'fopen' function after this one.

    char * e = dlerror(); // e will be NULL if no error occured, or a string describing the error if one did occur.
    if (e != NULL) { 
        puts(e);
        exit(1);
    }

    /*
    Theoretically dlsym could return NULL without error as symbols can have address zero.
    We wouldn't want to actually call the function if that was the case though.
    */
    if (libc_fopen == NULL) {
        puts("NSOD: open symbol with address NULL.");
        exit(1); 
    }

    return (libc_fopen(path, mode));
}


FILE * fopen(const char * path, const char * mode) {

    // Only hook if in readonly mode, as we have no way to handle creating/writing to an actual file.
    if (strncmp(mode, "r", 2) != 0) { 
        return __nsod_fallback_fopen(path, mode);
    }


    // It's worth doing this before we know if we're actually doing the hook to avoid having to parse cfg twice.
    int pipe_fds[2];
    if (pipe(pipe_fds) != 0) {
        return(NULL); // Non-normal return. Errno will be set by the pipe() function.
    }
    int pipe_write = pipe_fds[1];
    int pipe_read = pipe_fds[0];
        
    
    int rust_result = __nsod_rust_request(path, pipe_write);

    // Handle results from rust function call:
    if (rust_result != 0) { // Non-normal return.
        close(pipe_read);
        return __nsod_fallback_fopen(path, mode);
    }

    return(fdopen(pipe_read, "r")); // Normal return: return FILE from fd, filled with secret.
}