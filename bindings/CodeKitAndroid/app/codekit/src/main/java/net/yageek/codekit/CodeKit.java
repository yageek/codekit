package net.yageek.codekit;

public class CodeKit {
    static  {
        System.loadLibrary("codekit_core");
    }
    native static String makeEAN8(String code) ;
}
