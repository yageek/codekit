package net.yageek.codekit;

/**
 * The options used to generate
 * a barcode
 */
public class CodeOptions {
    private final int codeHeight;
    private final int quietSpace;
    private final int borderWidth;

    /**
     * Options to create a bar code
     * @param codeHeight The height of the code
     * @param quietSpace The size of the quiet space
     * @param borderWidth The size of the border width
     */
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
