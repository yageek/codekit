# Getting Started

## Register the filters

The first step is to register the filters into the system. On iOS, best ways would probably be 
in the applications delegate.

In objective-c:

```objc
#import<CodeKit/CodeKit.h>

/// Registering the filters
[[RCKCodeKit sharedInstance] registerFilters];
```

In swift:
```swift
import CodeKit

RCKCodeKit.shared.registerFilters()
```

## Using a filter

For every filter, you can either use the safe API or the dynamic API.

### Safe API

See the different methods and protocol

- ``RCKCodeGenerator``
- ``RCKCodeKit/ean8GeneratorFilter``
- ``RCKCodeKit/ean13GeneratorFilter``
- ``RCKCodeKit/code39GeneratorFilter``
- ``RCKCodeKit/code93GeneratorFilter``
- ``RCKCodeKit/codabarGeneratorFilter``
- ``RCKCodeKit/i2of5GeneratorFilter``

As an example, let's create a Code39 code:

```objc
CIFilter<RCKCodeGenerator> *filter = [RCKCodeKit code93GeneratorFilter];
filter.codeValue = @"TEST93";
CIImage *image = filter.outputImage;
```
### Dynamic API

The filters can be used using the standard CoreImage filter system.
The name of the different filters are:

- `RCKEAN8CodeGenerator`
- `RCKEAN13CodeGenerator` 
- `RCKCode39CodeGenerator`
- `RCKICode93odeGenerator` 
- `RCKI2of5CodeGenerator`

Each of this filter has the following properties as float elements to configure the shape
of the code:

- `borderWidth`: The border of the code. Defaults to 0.
- `quietSpace`: The quiet space of the code. Default to 7.
- `barCodeHeight`: The bar code height. Default to 50.

To setup the value of the code, use the string parameter `codeValue`.

As an example, let's create a Code39 code:

```objc
CIFilter *filter = [CIFilter filterWithName:@"RCKCode93CodeGenerator"];
[filter setValue:@"TEST93" forKey:@"codeValue"];

CIImage *image = filter.outputImage;
```

### Render to an UIImage

As we are using classic CoreImage filters, we can generate a `UIImage` using 
the regular API:

```objc
CIFilter<RCKCodeGenerator> *filter = [RCKCodeKit code93GeneratorFilter];
filter.codeValue = @"TEST93";
CIImage *image = filter.outputImage;

CGFloat scaleX = CGRectGetWidth(self.imageView.frame)/CGRectGetWidth(image.extent);
CGAffineTransform tr = CGAffineTransformScale(CGAffineTransformIdentity, scaleX, scaleX);
CIImage *scaled = [image imageByApplyingTransform:tr];
UIImage *uiImage = [UIImage imageWithCIImage:scaled];
self.imageView.image = uiImage;
```
