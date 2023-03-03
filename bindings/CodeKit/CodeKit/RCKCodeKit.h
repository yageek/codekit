//
//  RCKCodeKit.h
//  CodeKit
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import <CodeKit/CodeKit.h>
#import <CoreImage/CoreImage.h>
NS_ASSUME_NONNULL_BEGIN

/// The interface implementing ``CIFilterContructor``
@interface RCKCodeKit : NSObject <CIFilterConstructor>

/// Register the filters into CoreImage.
-(void) registerFilters;
@end

NS_ASSUME_NONNULL_END
