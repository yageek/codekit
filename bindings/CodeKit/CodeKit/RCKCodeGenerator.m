//
//  RCKCodeGenerator.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeGenerator.h"
#import "RCKCodeDescriptor+Private.h"
@implementation RCKCodeGenerator


- (CIImage *)outputImage {
	CGImageRef image = [self outputCGImage];
	if (!image) {
		return nil;
	}
	
	return [[CIImage alloc] initWithCGImage:image options:@{kCIImageNearestSampling: @YES}];
}

-(CGImageRef) outputCGImage {
	
	if (!self.barCodeDescriptor) { return nil; }
	
	NSInteger totalBar = self.barCodeDescriptor.barsCount;
	NSInteger bytesPerPixel = 4;
	NSInteger barcodeHeight = (NSInteger)self.barCodeDescriptor.barCodeHeight;
	NSInteger borderWidth = (NSInteger) self.barCodeDescriptor.borderWidth;
	NSInteger quietSpace = (NSInteger)self.barCodeDescriptor.quietSpace;
	
	NSInteger totalHeight = barcodeHeight + 2*quietSpace + 2*borderWidth;
	NSInteger totalWidth = totalBar + 2*quietSpace + 2*borderWidth;
	
	NSInteger totalBytesPerLine = totalWidth*bytesPerPixel;
	UInt8 *emptyLine = malloc(totalBytesPerLine);
	memset(emptyLine, 0xff, totalBytesPerLine);
	
	UInt8 *borderLine = malloc(totalBytesPerLine);
	memset(borderLine, 0x00, totalBytesPerLine);
	
	CFMutableDataRef data = CFDataCreateMutable(NULL, totalBytesPerLine);
	
	// Top Border
	for (NSInteger i = 0; i < borderWidth; i++) {
		CFDataAppendBytes(data, borderLine, totalBytesPerLine);
	}
	
	// Top quiet
	for (NSInteger i = 0; i < quietSpace; i++) {
		CFDataAppendBytes(data, emptyLine, totalBytesPerLine);
	}
	
	// BarCode line pointer - We take the existing UInt32 and prepend quiet space
	CFMutableDataRef codeLine = CFDataCreateMutable(nil, totalBytesPerLine);
	
	// We add the left border
	CFDataAppendBytes(codeLine, borderLine, borderWidth*bytesPerPixel);
	
	// We add the left spacing
	CFDataAppendBytes(codeLine, emptyLine, quietSpace*bytesPerPixel);
	
	// We add the bar elemeent
	CFDataAppendBytes(codeLine, self.barCodeDescriptor.headPointer, self.barCodeDescriptor.barsCount);
	
	// We add the right spacing
	CFDataAppendBytes(codeLine, emptyLine, quietSpace*bytesPerPixel);

	// We add right border
	CFDataAppendBytes(codeLine, borderLine, borderWidth*bytesPerPixel);
	
	// Now that we have one full line for the code,
	// we will it
	for (NSInteger i = 0; i < self.barCodeDescriptor.barCodeHeight; i++) {
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
	
	CGImageRef image = CGImageCreate(totalWidth, totalHeight, 8, 32, totalBytesPerLine, CGColorSpaceCreateDeviceRGB(), order | kCGImageAlphaNoneSkipLast , provider, NULL, NO, kCGRenderingIntentAbsoluteColorimetric);
	
	return image;
}

- (NSDictionary *)customAttributes
{
	return @{
		@"barCodeDescriptor": @{
			kCIAttributeName: @"barcodeDescriptor",
			kCIAttributeDisplayName: @"The bar code descriptor",
			kCIAttributeClass: @"RCKCodeDescriptor",
			kCIAttributeFilterCategories: @[kCICategoryGenerator]
		}
   };
}

@end
