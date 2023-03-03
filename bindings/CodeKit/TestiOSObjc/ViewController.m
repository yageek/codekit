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
}


@end
