====== NSOD: No Secrets On Disk ======
NSOD is a C library hook designed to allow older third-party programs to follow modern security practices by eliminating hardcoded storage of unencypted credentials on disk.

Functionality:

    NSOD intercepts all attempts to open files via the libc open() family of functions.

    
    If the path about to be opened matches an entry in NSOD's configuration, the file will never be opened.
    Instead, the calling program will receive a file descriptor containing data from another source.
    This data is buffered in the kernel and never written to disk (https://man7.org/linux/man-pages/man2/pipe.2.html).

    Sources include:

        Environment variable: simply include the secret in the specified environment variable.

        HashiCorp Vault: the hooked program will build a Vault client based on data in NSOD's config and attempt to retrieve the secret. Credentials (i.e. Vault tokens/certificates/passwords) must be injected via any of the other sources.
        
        File: retrieve the secret from a file other than the one originally opened. Allows circumventing hardcoded paths to read from more secure sources, such as ephemeral filesystems.

    When the opened path is not included in NSOD's configuration, the file is opened as usual.
    This also occurs when the open mode is set to anything other than readonly.


Compatibility:

    NSOD is Linux exclusive and relies on the LD_PRELOAD environment variable.
    This means it is INCOMPATIBLE with setuid programs.

    NSOD was designed for Nginx but can hook any program using the C library functions open() and open64().
    This naturally includes most programs written in C and C++, but many other languages also use the C library.
    Debugging tools such as ltrace and strace may help you determine if a given program uses these functions.

Security:

    NSOD relies on LD_PRELOAD. While LD_PRELOAD does not allow for privilege escalation, it is used in some persistance techniques.
    These techniques are not made *possible* by LD_PRELOAD, but they are made *easier*.
    As a result, LD_PRELOAD is sometimes disabled on systems where security is a priority.
    It is up to the user to decide whether this risk outweighs the benefits of more secure credential storage.
    
Performance:

    When using Vault as a source, NSOD has to retrieve the secret from the central Vault server. (???)
    While this may be desirable behaviour in many circumstances, it can easily become a bottleneck, especially if the same secret is accessed multiple times.
    In this situation, it is recommended to retrieve the secret from Vault externally and then send it into NSOD through another source.

    Performance impacts should otherwise be negligible unless thousands of open() calls are being made. (???)
    Performance may actually be improved (potentially greatly) if NSOD intercepts open() calls that would otherwise lead to slow reads off of disk.