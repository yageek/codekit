package net.yageek.codekit;

public class CodeOptions {

    private final int codeHeight;
    private final int quietSpace;
    private final int borderWidth;

    public CodeOptions(int codeHeight, int quietSpace, int borderWidth) {
        this.codeHeight = codeHeight;
        this.quietSpace = quietSpace;
        this.borderWidth = borderWidth;
    }

    public int getCodeHeight() {
        return codeHeight;
    }

    public int getQuietSpace() {
        return quietSpace;
    }

    public int getBorderWidth() {
        return borderWidth;
    }
}
