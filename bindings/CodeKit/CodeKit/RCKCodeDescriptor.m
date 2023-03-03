//
//  RCKCodeDescriptor.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeDescriptor.h"
#import "CodeKit-Core.h"

@implementation RCKCodeDescriptor
@synthesize data = _data, borderWidth = _borderWidth, barCodeHeight = _barCodeHeight, quietSpace = _quietSpace;
-(instancetype) initWithCoreDescriptor:(CodeKitCodeDescriptor)descriptor {
	self = [super init];
	
	if (self) {
		
		_data = [NSData dataWithBytes:descriptor.bars length:descriptor.bars_count];
		_borderWidth = descriptor.options.border_width;
		_quietSpace = descriptor.options.quiet_space;
		_barCodeHeight = descriptor.options.code_height;
	}
	return self;
}
#pragma mark - Getter

@end
