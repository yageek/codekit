//
//  RCKCodeDescriptor+Private.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "RCKCodeDescriptor.h"

NS_ASSUME_NONNULL_BEGIN
typedef struct CodeKitCodeDescriptor CodeKitCodeDescriptor;

@interface RCKCodeDescriptor (Private)
-(instancetype) initWithCoreDescriptor:(CodeKitCodeDescriptor*)descriptor;

@property(nonatomic, assign, readonly) NSInteger barsCount;
-(const uint8_t*) headPointer;
@end

NS_ASSUME_NONNULL_END
