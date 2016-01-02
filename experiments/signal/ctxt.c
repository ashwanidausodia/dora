#include <stdio.h>
#include <stdint.h>
#include <signal.h>
#include <unistd.h>
#include <sys/mman.h>
#include <string.h>
#include <ucontext.h>

#define REG_RIP 16

typedef int (*ftype)();

ftype alloc_code(const char *code, size_t len) {
  long pagesize;

  if ((pagesize = sysconf(_SC_PAGESIZE)) == -1) {
    perror("sysconf(_SC_PAGESIZE) failed");
  }

  void *ptr = mmap(NULL, pagesize, PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_ANON | MAP_PRIVATE, -1, 0);

  memcpy(ptr, code, len);
  return ptr;
}

void dump(const char *name, void *ptr, size_t len) {
  uint8_t *byte_ptr = (uint8_t *) ptr;
  printf("%s @ %p (%d bytes) = ", name, byte_ptr, len);

  for(size_t i=0; i<len; i++) {
    printf("%02x ", byte_ptr[i]);
  }

  printf("\n");
}

void handler(int signo, siginfo_t *info, void *context) {
  ucontext_t *ucontext = context;
  mcontext_t *mcontext = &ucontext->uc_mcontext;

  printf("signal %d!\n", signo);

  uint8_t *xpc = (uint8_t*) mcontext->gregs[REG_RIP];
  printf("program counter = %p\n", xpc);

  dump("program counter", xpc, 8);

  // mov eax, 4
  unsigned char fct1_code[] = { 0xb8, 4, 0, 0, 0, 0xc3 };
  ftype fct1 = alloc_code(fct1_code, sizeof(fct1_code));

  mcontext->gregs[REG_RIP] = (greg_t) fct1;
}

int main(int argc, char *argv[]) {
  struct sigaction sa;

  sa.sa_sigaction = handler;
  sigemptyset(&sa.sa_mask);
  sa.sa_flags = SA_SIGINFO;

  if (sigaction(SIGSEGV, &sa, NULL) == -1) {
    perror("sigaction failed");
  }

  // int fct2 { .... }
  // int fct1() { return fct2(); }

  // compiler stub: mov r10, [9]
  unsigned char fct2_stub[] = { 0x4C, 0x8B, 0x14, 0x25, 9, 0, 0, 0 };
  ftype fct2 = alloc_code(fct2_stub, sizeof(fct2_stub));

  dump("fct2_stub", fct2, sizeof(fct2_stub));

  // int3
  // movabs rax, 0x1122334455667788
  // call [rax]
  // ret
  uint8_t fct1_code[] = {
    0xCC,
    0x48, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0,
    0xFF, 0x10,
    0xC3
  };
  uint8_t *fct1_start = fct1_code;
  size_t fct1_code_length = sizeof(fct1_code);

  intptr_t *fct1_addr = (intptr_t *) (fct1_code + 3);

  if (argc <= 1 || strcmp(argv[1], "--with-int3") != 0) {
    fct1_start++;
    fct1_code_length--;
  }

  *(fct1_addr) = (intptr_t) fct2;

  ftype fct1 = alloc_code(fct1_start, fct1_code_length);
  dump("fct1", fct1, fct1_code_length);

  printf("invoke fct1:\n");
  int res1 = fct1();
  printf("res = %d\n", res1);

  // invoke fct a second time - stub should not be used anymore
  printf("\ninvoke fct1 again:\n");
  int res2 = fct1();
  printf("res = %d\n", res2);

  return 0;
}
