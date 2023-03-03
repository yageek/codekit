package net.yageek.codekit;

/**
 * The options used to generate
 * a barcode
 */
public class CodeOptions {
    private final int codeHeight;
    private final int quietSpace;
    private final int borderWidth;
    private final int barWidth;

    /**
     * Options to create a bar code
     * @param codeHeight The height of the code
     * @param quietSpace The size of the quiet space
     * @param borderWidth The size of the border width
     * @param barWidth The size of the bar width
     */
    public CodeOptions(int codeHeight, int quietSpace, int borderWidth, int barWidth) {
        this.codeHeight = codeHeight;
        this.quietSpace = quietSpace;
        this.borderWidth = borderWidth;
        this.barWidth = barWidth;
    }

    /**
     * Options to create a bar code with a bar width of 1
     * @param codeHeight The height of the code
     * @param quietSpace The size of the quiet space
     * @param borderWidth The size of the border width
     */
    public CodeOptions(int codeHeight, int quietSpace, int borderWidth) {
        this.codeHeight = codeHeight;
        this.quietSpace = quietSpace;
        this.borderWidth = borderWidth;
        this.barWidth = 1;
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

    public int getBarWidth() {
        return barWidth;
    }
}
