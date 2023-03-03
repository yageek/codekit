#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A descriptors holding all the
 * informations to draw a code
 */
typedef struct CodeKitCodeDescriptor {
  /**
   * A pointer in memory to an array
   * of byte where each one represent either a blank (0) or black (1) bar
   */
  uint8_t *bars;
  /**
   * The total number of bars stored in memory
   */
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
                                struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for EAN8 code
 */
int8_t codekit_code_create_ean13(const char *content,
                                 struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for a Code39 code.
 */
int8_t codekit_code_create_code39(const char *content,
                                  struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for a Code93 code.
 */
int8_t codekit_code_create_code93(const char *content,
                                  struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for a Codabar code.
 */
int8_t codekit_code_create_codabar(const char *content,
                                   struct CodeKitCodeDescriptor *value);

/**
 * Create a descriptor for a Interleaved code.
 */
int8_t codekit_code_create_i2of5(const char *content,
                                 struct CodeKitCodeDescriptor *value);
