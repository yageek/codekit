//
//  RCKCodeGenerator.m
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeGenerator.h"
#import "RCKCodeDescriptor+Private.h"
@implementation RCKCodeGenerator
@synthesize barCodeDescriptor = _barCodeDescriptor;

- (CIImage *)outputImage {
	CGImageRef image = [self outputCGImage];
	if (!image) {
		return nil;
	}
	
	return [[CIImage alloc] initWithCGImage:image options:@{kCIImageNearestSampling: @YES}];
}

-(CGImageRef) outputCGImage {
	
	if (!self.barCodeDescriptor) { return nil; }
	
	NSInteger totalBar = self.barCodeDescriptor.data.length;
	NSInteger bytesPerPixel = 4;
	NSInteger barcodeHeight = (NSInteger)self.barCodeDescriptor.barCodeHeight;
	NSInteger borderWidth = (NSInteger) self.barCodeDescriptor.borderWidth;
	NSInteger quietSpace = (NSInteger)self.barCodeDescriptor.quietSpace;
	
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
	CFDataAppendBytes(codeLine, &borderLine[0], borderWidth*bytesPerPixel);
	
	// We add the left spacing
	CFDataAppendBytes(codeLine, &emptyLine[0], quietSpace*bytesPerPixel);
	
	// We add the bar elemeent
	UInt32 buff;
	UInt8 value;
	for(NSInteger i = 0; i < self.barCodeDescriptor.data.length; i++) {
		[self.barCodeDescriptor.data getBytes:&value range:NSMakeRange(i, 1)];
		
		buff = value == 1 ? 0x0: 0xffffffff;
		CFDataAppendBytes(codeLine, (UInt8*)&buff, 4);
		
	}

	NSData *converted = (__bridge NSData*) codeLine;
	NSLog(@"Data: %@", converted);
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

@end
