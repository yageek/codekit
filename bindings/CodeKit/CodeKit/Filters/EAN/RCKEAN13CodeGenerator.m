//
//  RCKEAN16CodeGenerator.m
//  CodeKit
//
//  Created by Heinrich Yannick on 21/09/2022.
//

#import "RCKEAN13CodeGenerator.h"
#import "CodeKit-Core.h"

@implementation RCKEAN13CodeGenerator
- (void)setCodeValue:(NSString *)codeValue {

	_codeValue = [codeValue copy];
	
	CodeKitCodeDescriptor desc;
	CodeKitCodeOptions opts = {.code_height = self.barCodeHeight, .quiet_space = self.quietSpace, .border_width = self.borderWidth };
	NSInteger res = codekit_code_create_EAN13([codeValue cStringUsingEncoding:NSUTF8StringEncoding], opts, &desc);
	if (res < 0) {
		[self raiseErrorForCode:res];
	}
	
	self.data = [[NSData alloc] initWithBytes:desc.bars length:desc.bars_count];

	codekit_free_descriptor(&desc);
}
- (NSDictionary *)customAttributes
{
	
	NSMutableDictionary *dict = [[super customAttributes] mutableCopy];
	
	dict[@"codeValue"] = @{
			kCIAttributeClass: [NSString class],
			kCIAttributeDisplayName: @"Code",
			kCIAttributeDescription: @"The EAN13 code to convert to bar code",
   };
	return dict;
}
@end
