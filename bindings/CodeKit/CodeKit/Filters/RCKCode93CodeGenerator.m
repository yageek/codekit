//
//  RCKCode93CodeGenerator.m
//  CodeKit
//
//  Created by Heinrich Yannick on 13/10/2022.
//

#import "RCKCode93CodeGenerator.h"
#import "CodeKit-Core.h"

@implementation RCKCode93CodeGenerator
-(void)setCodeValue:(NSString*) codeValue {
    _codeValue = [codeValue copy];
    
    CodeKitCodeDescriptor desc;
    NSInteger res = codekit_code_create_code93([codeValue cStringUsingEncoding:NSUTF8StringEncoding], &desc);
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
            kCIAttributeDescription: @"The Code 93 code to convert to bar code",
   };
    return dict;
}
@end
