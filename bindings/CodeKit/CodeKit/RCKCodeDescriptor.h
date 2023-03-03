//
//  RCKCodeDescriptor.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface RCKCodeDescriptor : NSObject {
	NSData *_data;
	CGFloat _borderWidth;
	CGFloat _quietSpace;
	CGFloat _barCodeHeight;
}

@property(nonatomic, assign, readonly) CGFloat borderWidth;
@property(nonatomic, assign, readonly) CGFloat quietSpace;
@property(nonatomic, assign, readonly) CGFloat barCodeHeight;
@property(nonatomic, copy, readonly) NSData *data;
@end

NS_ASSUME_NONNULL_END
