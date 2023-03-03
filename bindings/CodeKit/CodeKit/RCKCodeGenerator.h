//
//  RCKCodeGenerator.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <CoreImage/CoreImage.h>

/// The protocol implemented by all internals elements
@protocol RCKCodeGenerator <CIFilter>

/// The border width of the code
@property(nonatomic, assign) NSInteger borderWidth;

/// The quiet space of the code
@property(nonatomic, assign) NSInteger quietSpace;

/// The barcode height
@property(nonatomic, assign) NSInteger barCodeHeight;

/// The code value
@property(nonatomic, copy) NSString *codeValue;
@end

/// Main superclass for all filter
/// based on one ``RCKCodeDescriptor``
@interface RCKCodeGenerator : CIFilter

@property(nonatomic, assign) NSInteger borderWidth;
@property(nonatomic, assign) NSInteger quietSpace;
@property(nonatomic, assign) NSInteger barCodeHeight;
@property(nonatomic, copy) NSData *data;

-(void) raiseErrorForCode:(NSInteger) code;
- (NSDictionary *)customAttributes;
@end

