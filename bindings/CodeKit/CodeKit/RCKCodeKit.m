//
//  RCKCodeKit.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeKit.h"
#import "RCKEAN8CodeGenerator.h"
@implementation RCKCodeKit 
- (void)registerFilters {
	// Initialize
	[CIFilter registerFilterName:@"RCKEAN8CodeGenerator" constructor:self classAttributes:@{
		kCIAttributeFilterDisplayName: @"EAN8 Barcode generator",
		kCIAttributeFilterCategories: @[kCICategoryGenerator]
	}];
}
- (CIFilter *)filterWithName:(NSString *)name {
	return [[RCKEAN8CodeGenerator alloc] init];
}
@end
