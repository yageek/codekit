//
//  ViewController.m
//  TestiOSObjc
//
//  Created by Heinrich Yannick on 20/09/2022.
//

#import "ViewController.h"
#import <CodeKit/CodeKit.h>

@interface ViewController ()
@property (weak, nonatomic) IBOutlet UIImageView *imageView;

@end

@implementation ViewController

- (void)viewDidLoad {
	[super viewDidLoad];
	// Do any additional setup after loading the view.
	
	self.imageView.contentMode = UIViewContentModeScaleAspectFit;
	
	CIFilter *filter = [CIFilter filterWithName:@"RCKCode93CodeGenerator"];
	[filter setValue:@"TEST93" forKey:@"codeValue"];
	
	CIImage *image = filter.outputImage;
	
	CGFloat scaleX = CGRectGetWidth(self.imageView.frame)/CGRectGetWidth(image.extent);
	CGAffineTransform tr = CGAffineTransformScale(CGAffineTransformIdentity, scaleX, scaleX);
	CIImage *scaled = [image imageByApplyingTransform:tr];
	UIImage *uiImage = [UIImage imageWithCIImage:scaled];
	self.imageView.image = uiImage;
}


@end
