//
//  RCKCodeKit.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <CodeKit/CodeKit.h>
#import <CoreImage/CoreImage.h>
#import <CodeKit/RCKCodeGenerator.h>
NS_ASSUME_NONNULL_BEGIN

/// The interface implementing ``CIFilterContructor``
@interface RCKCodeKit : NSObject <CIFilterConstructor>

/// Register the filters into CoreImage.
-(void) registerFilters;

///:nodoc:
- (instancetype) init NS_UNAVAILABLE;

/// The shared instance used to initialise the difference filters.
@property(nonatomic, class, strong, readonly) RCKCodeKit *sharedInstance NS_SWIFT_NAME(shared);

/// Get an EAN8 generator filter
+ (CIFilter<RCKCodeGenerator> *) ean8GeneratorFilter NS_SWIFT_NAME(ean8Generator());

/// Get an EAN13 generator filter
+ (CIFilter<RCKCodeGenerator> *) ean13GeneratorFilter NS_SWIFT_NAME(ean13Generator());

/// Get an Code39 generator filter
+ (CIFilter<RCKCodeGenerator> *) code39GeneratorFilter NS_SWIFT_NAME(code39Generator());

/// Get an Code93 generator filter
+ (CIFilter<RCKCodeGenerator> *) code93GeneratorFilter NS_SWIFT_NAME(code93Generator());

/// Get an Codabar generator filter
+ (CIFilter<RCKCodeGenerator> *) codabarGeneratorFilter NS_SWIFT_NAME(codabarGenerator());

/// Get an I2of5 generator filter
+ (CIFilter<RCKCodeGenerator> *) i2of5GeneratorFilter NS_SWIFT_NAME(i2of5Generator());

@end

NS_ASSUME_NONNULL_END
