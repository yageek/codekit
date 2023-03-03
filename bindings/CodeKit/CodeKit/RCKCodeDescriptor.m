//
//  RCKCodeDescriptor.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeDescriptor.h"
#import "CodeKit-Core.h"

@implementation RCKCodeDescriptor {
	CodeKitCodeDescriptor *_descriptor;
}

-(instancetype) initWithCoreDescriptor:(CodeKitCodeDescriptor*)descriptor {
	
	self = [super init];
	
	if (self) {
		
		_descriptor = descriptor;
	}
	return self;
}
- (void)dealloc {
	codekit_free_descriptor(_descriptor);
}
#pragma mark - Getter

- (CGFloat)borderWidth {
	return _descriptor->options.border_width;
}

- (CGFloat)barCodeHeight {
	return _descriptor->options.code_height;
}
- (CGFloat)quietSpace {
	return _descriptor->options.quiet_space;
}
#pragma mark - Private
-(NSInteger) barsCount {
	return _descriptor->bars_count;
}

-(const uint8_t*) headPointer {
	return _descriptor->bars;
}

@end
