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
@end

NS_ASSUME_NONNULL_END
