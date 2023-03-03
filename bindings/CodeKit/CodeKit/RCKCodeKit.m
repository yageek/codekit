//
//  RCKCodeKit.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeKit.h"
#import "RCKEAN8CodeGenerator.h"
#import "RCKEAN13CodeGenerator.h"
#import "RCKCode39CodeGenerator.h"
#import "RCKCode93CodeGenerator.h"
#import "RCKCodabarCodeGenerator.h"
#import "RCKI2of5CodeGenerator.h"

@implementation RCKCodeKit

#pragma mark - Init
+ (RCKCodeKit *)sharedInstance {
    static dispatch_once_t onceToken;
    static RCKCodeKit *instance;
    dispatch_once(&onceToken, ^{
        instance = [[RCKCodeKit alloc] init];
    });
    return instance;
}

- (void)registerFilters {
    // Initialize
    [CIFilter registerFilterName:@"RCKEAN8CodeGenerator" constructor:self classAttributes:@{
        kCIAttributeFilterDisplayName: @"EAN8 Barcode generator",
        kCIAttributeFilterCategories: @[kCICategoryGenerator]
    }];
    
    [CIFilter registerFilterName:@"RCKEAN13CodeGenerator" constructor:self classAttributes:@{
        kCIAttributeFilterDisplayName: @"EAN13 Barcode generator",
        kCIAttributeFilterCategories: @[kCICategoryGenerator]
    }];
    
    [CIFilter registerFilterName:@"RCKCode93CodeGenerator" constructor:self classAttributes:@{
        kCIAttributeFilterDisplayName: @"Code39 Barcode generator",
        kCIAttributeFilterCategories: @[kCICategoryGenerator]
    }];
    
    [CIFilter registerFilterName:@"RCKCode93CodeGenerator" constructor:self classAttributes:@{
        kCIAttributeFilterDisplayName: @"Code93 Barcode generator",
        kCIAttributeFilterCategories: @[kCICategoryGenerator]
    }];
    
    [CIFilter registerFilterName:@"RCKCodabarCodeGenerator" constructor:self classAttributes:@{
        kCIAttributeFilterDisplayName: @"Codabar Barcode generator",
        kCIAttributeFilterCategories: @[kCICategoryGenerator]
    }];
    
    
    [CIFilter registerFilterName:@"RCKI2of5CodeGenerator" constructor:self classAttributes:@{
        kCIAttributeFilterDisplayName: @"I2of5 Barcode generator",
        kCIAttributeFilterCategories: @[kCICategoryGenerator]
    }];
}

#pragma mark - CIFilterConstructor
- (CIFilter *)filterWithName:(NSString *)name {
    
    if ([name isEqualToString:@"RCKEAN8CodeGenerator"]){
        return [[RCKEAN8CodeGenerator alloc] init];
    } else if ([name isEqualToString:@"RCKEAN13CodeGenerator"]) {
        return [[RCKEAN13CodeGenerator alloc] init];
    } else if ([name isEqualToString:@"RCKCode39CodeGenerator"]) {
        return [[RCKCode39CodeGenerator alloc] init];
    } else if ([name isEqualToString:@"RCKCode93CodeGenerator"]) {
        return [[RCKCode93CodeGenerator alloc] init];
    } else if([name isEqualToString:@"RCKCodabarCodeGenerator"]) {
        return [[RCKCodabarCodeGenerator alloc] init];
    } else if([name isEqualToString:@"RCKI2of5CodeGenerator"]) {
        return [[RCKI2of5CodeGenerator alloc] init];
    }
    return nil;
    
}

#pragma mark - Properties
+ (CIFilter<RCKCodeGenerator> *) ean8GeneratorFilter {
    return [[RCKEAN8CodeGenerator alloc] init];
}

+ (CIFilter<RCKCodeGenerator> *) ean13GeneratorFilter {
    return [[RCKEAN13CodeGenerator alloc] init];
}

+ (CIFilter<RCKCodeGenerator> *) code39GeneratorFilter {
    return [[RCKCode39CodeGenerator alloc] init];
}

+ (CIFilter<RCKCodeGenerator> *) code93GeneratorFilter {
    return [[RCKCode93CodeGenerator alloc] init];
}

+ (CIFilter<RCKCodeGenerator> *) codabarGeneratorFilter {
    return [[RCKCodabarCodeGenerator alloc] init];
}

+ (CIFilter<RCKCodeGenerator> *) i2of5GeneratorFilter {
    return [[RCKI2of5CodeGenerator alloc] init];
}

@end
