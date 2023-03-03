//
//  RCKCodeGenerator.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <CoreImage/CoreImage.h>
#import "RCKCodeKit.h"
/// Base superclass of all generator filter
@interface RCKCodeGenerator : CIFilter

/// The border width of the code
@property(nonatomic, assign) NSInteger borderWidth;

/// The quiet space of the code
@property(nonatomic, assign) NSInteger quietSpace;

/// The barcode height
@property(nonatomic, assign) NSInteger barCodeHeight;

@property(nonatomic, copy) NSData *data;

-(void) raiseErrorForCode:(NSInteger) code;
- (NSDictionary *)customAttributes;
@end

