# CodeKit

A toolkit to generate 1D bar code. The generation logic is done on Rust and then wrapped into
different platforms/language using FFI or JNI.

You can generate the following types of codes:

- EAN8
- EAN13
- Code39
- Code93
- Codabar

The supported platform:

- iOS/macOS
- Android

## iOS/macOS

A xcframework and docc module are provided. See the releases page to download them.

### Register the filters

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

/// Registering the filters
RCKCodeKit.shared.registerFilters()
```

### Using a filter

For every filter, you can either use the safe API or the dynamic API.

### Safe API

See the different methods and protocol

- `RCKCodeGenerator`
- `RCKCodeKit/ean8GeneratorFilter`
- `RCKCodeKit/ean13GeneratorFilter`
- `RCKCodeKit/code39GeneratorFilter`
- `RCKCodeKit/code93GeneratorFilter`
- `RCKCodeKit/codabarGeneratorFilter`
- `RCKCodeKit/i2of5GeneratorFilter`

As an example, let's create a Code39 code:

```objc
CIFilter<RCKCodeGenerator> *filter = [RCKCodeKit code93GeneratorFilter];
filter.codeValue = @"TEST93";
CIImage *image = filter.outputImage;
```

```swift
let filter = RCKCodeKit.code93Generator()
filter.codeValue = "TEST93"
let image = filter.outputImage
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

### Android

A android AAR element is provided also in the release page.

The library contains functions to create a string of bars (0/1) from a java string
and one function to create a bitmap from those strings.

The available methods are the following:

```java
/**
* Generate an EAN8 code
* @param code A string representing the code
* @return A string representing the bars (1 for black and 0 for white)
*/
public native static String makeEAN8(String code);

/**
* Generate an EAN13 code
* @param code A string representing the code
* @return A string representing the bars (1 for black and 0 for white)
*/
public native static String makeEAN13(String code);

/**
* Generate an Codabar code
* @param code A string representing the code
* @return A string representing the bars (1 for black and 0 for white)
*/
public native static String makeCodabar(String code);

/**
* Generate an Code39 code
* @param code A string representing the code
* @return A string representing the bars (1 for black and 0 for white)
*/
public native static String makeCode39(String code);

/**
* Generate an Code93 code
* @param code A string representing the code
* @return A string representing the bars (1 for black and 0 for white)
*/
public native static String makeCode93(String code);

/**
* Generate an Interleave 2 of 5 code
* @param code A string representing the code
* @return A string representing the bars (1 for black and 0 for white)
*/
public native static String makeI2Of5(String code);

/**
* Generate a bitmap image from a CodeDescriptor
* @param descriptor The descriptor representing the code
* @return A string representing the bars (1 for black and 0 for white)
*/
public static Bitmap convertBitmap(String descriptor, CodeOptions options)
```

A basic usage could be done as the following:

```java
import net.yageek.codekit.CodeKit;
import net.yageek.codekit.CodeOptions;

// Creating the string of bars from the code
String test = CodeKit.makeCode93("TEST93");

// Creating a code with a height of 200, a quiet space of 7, no border and a bar width of 5
CodeOptions options = new CodeOptions(200, 7, 0, 5);
Bitmap bitmap = CodeKit.convertBitmap(test, options);
```
