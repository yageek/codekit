#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CodeKitVec_u8 CodeKitVec_u8;

typedef struct CodeKitCodeOptions {
  float quiet_space;
  float code_height;
  float border_width;
} CodeKitCodeOptions;

typedef struct CodeKitCodeDescriptor {
  struct CodeKitCodeOptions options;
  const uint8_t *bars;
  uintptr_t bars_count;
  struct CodeKitVec_u8 *inner;
} CodeKitCodeDescriptor;

void codekit_free_descriptor(struct CodeKitCodeDescriptor *ptr);

int8_t codekit_code_create_EAN8(const char *content,
                                struct CodeKitCodeOptions options,
                                struct CodeKitCodeDescriptor *value);
