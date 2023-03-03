//
//  RCKCodeDescriptor.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface RCKCodeDescriptor : NSObject

@property(nonatomic, assign, readonly) CGFloat borderWidth;
@property(nonatomic, assign, readonly) CGFloat quietSpace;
@property(nonatomic, assign, readonly) CGFloat barCodeHeight;
@end

NS_ASSUME_NONNULL_END
