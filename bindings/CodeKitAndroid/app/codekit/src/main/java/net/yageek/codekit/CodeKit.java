package net.yageek.codekit;

public class CodeKit {
    static  {
        System.loadLibrary("codekit_core");
    }
    native static CodeDescriptor makeEAN8(String code, CodeOptions options);
}
