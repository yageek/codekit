//
//  RCKCodeGenerator.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <CoreImage/CoreImage.h>

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

