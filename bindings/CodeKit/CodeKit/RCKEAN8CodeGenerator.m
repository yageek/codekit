//
//  _RCKEAN8CodeGenerator.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKEAN8CodeGenerator.h"
#import "RCKCodeDescriptor+Private.h"
#import "CodeKit-Core.h"
@implementation RCKEAN8CodeGenerator
#pragma mark - Init


- (RCKCodeDescriptor *)barCodeDescriptor {
	if (_barCodeDescriptor) return _barCodeDescriptor;
	
	CodeKitCodeDescriptor desc;
	CodeKitCodeOptions opts = {.code_height = 50.0, .quiet_space = 7.0, .border_width = 0.0};
	uint8_t res = codekit_code_create_EAN8([self.codeValue cStringUsingEncoding:NSUTF8StringEncoding], opts, &desc);
	if (res < 0) {
		[NSException raise:@"error" format:@"error"];
	}

	RCKCodeDescriptor *descriptor = [[RCKCodeDescriptor alloc] initWithCoreDescriptor:&desc];
	_barCodeDescriptor = descriptor;
	codekit_free_descriptor(&desc);
	
	return descriptor;
}
- (NSDictionary *)customAttributes
{
	return @{
		@"codeValue": @{
			kCIAttributeClass: [NSString class],
			kCIAttributeDisplayName: @"Code",
			kCIAttributeDescription: @"The EAN8 code to convert to bar code",
		}
   };
}

@end
