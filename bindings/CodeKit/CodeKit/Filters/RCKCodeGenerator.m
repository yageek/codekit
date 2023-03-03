//
//  RCKCodeGenerator.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeGenerator.h"

@implementation RCKCodeGenerator

-(instancetype) init {
    self = [super init];
    
    if (self) {
        self.barCodeHeight = 50.0;
        self.quietSpace = 7.0;
        self.borderWidth = 0.0;
    }
    return self;
}

- (CIImage *)outputImage {
    CGImageRef image = [self outputCGImage];
    if (!image) {
        return nil;
    }
    
    return [[CIImage alloc] initWithCGImage:image options:@{kCIImageNearestSampling: @YES}];
}

-(CGImageRef) outputCGImage {
    
    if (!self.data || self.data.length < 1) {
        return nil;
    }
    
    NSInteger totalBar = self.data.length;
    NSInteger bytesPerPixel = 4;
    NSInteger barcodeHeight = self.barCodeHeight;
    NSInteger borderWidth = self.borderWidth;
    NSInteger quietSpace = self.quietSpace;
    
    NSInteger totalHeight = barcodeHeight + 2*quietSpace + 2*borderWidth;
    NSInteger totalWidth = totalBar + 2*quietSpace + 2*borderWidth;
    
    NSInteger totalBytesPerLine = totalWidth*bytesPerPixel;
    UInt8 *emptyLine = malloc(totalBytesPerLine*sizeof(UInt8));
    memset(emptyLine, 0xff, totalBytesPerLine);
    
    UInt8 *borderLine = malloc(totalBytesPerLine*sizeof(UInt8));
    memset(borderLine, 0x00, totalBytesPerLine);
    
    CFMutableDataRef data = CFDataCreateMutable(NULL,  totalWidth*totalHeight*bytesPerPixel);
    
    for (NSInteger i = 0; i < borderWidth; i++) {
        CFDataAppendBytes(data, &borderLine[0], totalBytesPerLine);
    }
    
    // Top quiet
    for (NSInteger i = 0; i < quietSpace; i++) {
        CFDataAppendBytes(data, &emptyLine[0], totalBytesPerLine);
    }
    
    // BarCode line pointer - We take the existing UInt32 and prepend quiet space
    CFMutableDataRef codeLine = CFDataCreateMutable(NULL, totalBytesPerLine);
    
    // We add the left border
    CFDataAppendBytes(codeLine, &borderLine[0], borderWidth*bytesPerPixel); // WARNING
    
    // We add the left spacing
    CFDataAppendBytes(codeLine, &emptyLine[0], quietSpace*bytesPerPixel); // WARNING
    
    // We add the bar elemeent
    UInt32 buff;
    UInt8 value;
    for(NSInteger i = 0; i < self.data.length; i++) {
        [self.data getBytes:&value range:NSMakeRange(i, 1)];
        
        buff = value == '1' ? 0x0: 0xffffffff;
        CFDataAppendBytes(codeLine, (UInt8*)&buff, 4);
    }
    
    // We add the right spacing
    CFDataAppendBytes(codeLine, emptyLine, quietSpace*bytesPerPixel);
    
    // We add right border
    CFDataAppendBytes(codeLine, borderLine, borderWidth*bytesPerPixel);
    
    // Now that we have one full line for the code,
    // we will it
    for (NSInteger i = 0; i < barcodeHeight; i++) {
        
        CFDataAppendBytes(data, CFDataGetBytePtr(codeLine), totalWidth*bytesPerPixel);
    }
    // Now we add the last bottom quiet
    for (NSInteger i = 0; i < quietSpace; i++) {
        CFDataAppendBytes(data, emptyLine, totalWidth*bytesPerPixel);
    }
    
    // Bottom border
    for (NSInteger i = 0; i < borderWidth; i++) {
        CFDataAppendBytes(data, borderLine, totalWidth*bytesPerPixel);
    }
    
    CGDataProviderRef provider = CGDataProviderCreateWithCFData(data);
    
    CGBitmapInfo order = CFByteOrderGetCurrent() == CFByteOrderLittleEndian ? kCGBitmapByteOrder32Little : kCGBitmapByteOrder32Big;
    
    CGImageRef image = CGImageCreate(totalWidth, totalHeight, 8, 32, totalBytesPerLine, CGColorSpaceCreateDeviceRGB(), order | kCGImageAlphaNoneSkipLast, provider, NULL, NO, kCGRenderingIntentAbsoluteColorimetric);
    
    return image;
}
#pragma mark - Attributes
- (NSDictionary *)customAttributes
{
    return @{
        @"borderWidth": @{
            kCIAttributeMin       : @0,
            kCIAttributeType: kCIAttributeTypeScalar,
            kCIAttributeDisplayName: @"borderWidth",
            kCIAttributeDescription: @"The border width of the filter",
            kCIAttributeDefault: @0,
        },
        @"quietSpace": @{
            kCIAttributeMin       : @0,
            kCIAttributeDisplayName: @"quietSpace",
            kCIAttributeDescription: @"The quiet space for the code",
            kCIAttributeDefault: @7,
        },
        @"barCodeHeight": @{
            kCIAttributeMin       : @0,
            kCIAttributeDisplayName: @"barCodeHeight",
            kCIAttributeDescription: @"The height of the bar code",
            kCIAttributeDefault: @50,
        }
    };
}
#pragma mark - Exception management

- (void)raiseErrorForCode:(NSInteger)code {
    NSException *exception = [NSException exceptionWithName:@"CodeKitCoreException" reason:@"the internal core library failed to create an error" userInfo:nil];
    @throw exception;
}
@end
