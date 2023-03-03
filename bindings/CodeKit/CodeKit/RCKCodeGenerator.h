//
//  RCKCodeGenerator.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <CoreImage/CoreImage.h>
#import "RCKCodeDescriptor.h"

@interface RCKCodeGenerator : CIFilter
@property(nonatomic, strong, nullable) RCKCodeDescriptor* barCodeDescriptor;
@end

