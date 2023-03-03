//
//  RCKI2of5CodeGenerator.h
//  CodeKit
//
//  Created by Heinrich Yannick on 13/10/2022.
//

#import "RCKCodeGenerator.h"

NS_ASSUME_NONNULL_BEGIN

@interface RCKI2of5CodeGenerator : RCKCodeGenerator <RCKCodeGenerator>
@property(nonatomic, copy) NSString *codeValue;
@end

NS_ASSUME_NONNULL_END
