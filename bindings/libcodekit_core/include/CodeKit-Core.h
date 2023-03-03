#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct CodeKitCodeOptions {
  uint16_t quiet_space;
  uint16_t code_height;
  uint16_t border_width;
} CodeKitCodeOptions;

typedef struct CodeKitCodeDescriptor {
  struct CodeKitCodeOptions options;
  uint8_t *bars;
  uintptr_t bars_count;
} CodeKitCodeDescriptor;

/**
 * Free a code descriptor from a pointer
 * to CodeDecriptor
 */
void codekit_free_descriptor(struct CodeKitCodeDescriptor *ptr);

/**
 * Create a descriptor for EAN8 code
 */
int8_t codekit_code_create_ean8(const char *content,
                                struct CodeKitCodeOptions options,
                                struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for EAN8 code
 */
int8_t codekit_code_create_ean13(const char *content,
                                 struct CodeKitCodeOptions options,
                                 struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for a Code39 code.
 */
int8_t codekit_code_create_code39(const char *content,
                                  struct CodeKitCodeOptions options,
                                  struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for a Code93 code.
 */
int8_t codekit_code_create_code93(const char *content,
                                  struct CodeKitCodeOptions options,
                                  struct CodeKitCodeDescriptor *value);
