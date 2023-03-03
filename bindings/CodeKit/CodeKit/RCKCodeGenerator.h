//
//  RCKCodeGenerator.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <CoreImage/CoreImage.h>
#import <CodeKit/RCKCodeDescriptor.h>

@interface RCKCodeGenerator : CIFilter {
	RCKCodeDescriptor* _barCodeDescriptor;
}
@property(nonatomic, strong, nullable) RCKCodeDescriptor* barCodeDescriptor;
@end

