#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

extern char **environ;

int main(int argc, char *argv[], char *envp[]) {
  printf("Hello World!\n");

  /** ## Arguments ## */
  printf("Number of arguments (argc): %d\n", argc);
  printf("Arguments (argv):\n");

  for (int i = 0; i < argc; i++) {
    printf("argv[%d]: %s\n", i, argv[i]);
  }

  /** ## Environment ## */
  printf("Environment Variables starting with DEMO_ (using envp):\n");

  if (envp == NULL) {
    printf("Environment pointer (envp) is NULL.\n");
  } else {
    for (int i = 0; envp[i] != NULL; i++) {
      if (strncmp(envp[i], "DEMO_", 5) == 0) {
        printf("envp[%d]: %s\n", i, envp[i]);
      }
    }
  }

  printf("\nEnvironment Variables starting with DEMO_ (using environ):\n");

  if (environ == NULL) {
    printf("Environment pointer (environ) is NULL.\n");
  } else {
    for (int i = 0; environ[i] != NULL; i++) {
      if (strncmp(environ[i], "DEMO_", 5) == 0) {
        printf("environ[%d]: %s\n", i, environ[i]);
      }
    }
  }

  /* ## Memory ## */
  {
    void *a_buff = malloc(1024);
    void *b_buff = calloc(1024, 1);

    memset(a_buff, 42, 1024);
    memcpy(b_buff, a_buff, 1024);
    int buffcmp = memcmp(a_buff, b_buff, 1024);
    free(a_buff);
    free(b_buff);
    if (buffcmp != 0) {
      printf("Buffers are not equal\n");
      exit(EXIT_FAILURE);
    }
  }

  return EXIT_SUCCESS;
}
