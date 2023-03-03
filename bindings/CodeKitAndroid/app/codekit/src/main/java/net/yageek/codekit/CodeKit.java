package net.yageek.codekit;

import android.graphics.Bitmap;


import java.nio.IntBuffer;
import java.util.Arrays;

/**
 * The main class used to generate the different
 * bar codes.
 */
public class CodeKit {
    static  {
        System.loadLibrary("codekit_core");
    }

    private static final int WHITE_COLOR = 0xffffffff;
    private static final int BLACK_COLOR = 0xff000000;

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
    public static Bitmap convertBitmap(String descriptor, CodeOptions options) {

        final int totalBar = descriptor.length();
        final int barcodeHeight = options.getCodeHeight();
        final int borderWidth = options.getBorderWidth();
        final int quietSpace = options.getQuietSpace();

        final int totalHeight = barcodeHeight + 2*quietSpace + 2*borderWidth;
        final int totalWidth = totalBar*options.getBarWidth() + 2*quietSpace + 2*borderWidth;

        final int[] emptyLine = new int[totalWidth];
        Arrays.fill(emptyLine, WHITE_COLOR);

        final int[] borderLine = new int[totalWidth];
        Arrays.fill(borderLine, BLACK_COLOR);

        IntBuffer data = IntBuffer.allocate(totalHeight*totalWidth);

        for (int i = 0; i < borderWidth; i++) {
            data.put(borderLine, 0, totalWidth);
        }

        // Top quiet
        for (int i = 0; i < quietSpace; i++) {
            data.put(emptyLine, 0 , totalWidth);
        }

        // BarCode line pointer - We take the existing UInt32 and prepend quiet space
        IntBuffer codeLine = IntBuffer.allocate(totalWidth);

        // We add the left border
        codeLine.put(borderLine, 0, borderWidth); // WARNING

        // We add the left spacing
        codeLine.put(emptyLine, 0, quietSpace); // WARNING

        int blackColor [] = new int [options.getBarWidth()];
        Arrays.fill(blackColor, BLACK_COLOR);

        int whiteColor [] = new int [options.getBarWidth()];
        Arrays.fill(whiteColor, WHITE_COLOR);

        // We add the bar element
        for (int i = 0; i < totalBar; i++){
            char value = descriptor.charAt(i);

            if (value == '1') {
                codeLine.put(blackColor);
            } else {
                codeLine.put(whiteColor);
            }
        }

        // We add the right spacing
        codeLine.put(emptyLine, 0, quietSpace); // WARNING

        // We add the right border
        codeLine.put(borderLine, 0, borderWidth); // WARNING

        for ( int i = 0; i < barcodeHeight; i++) {
            data.put(codeLine.array());
        }

        // Top quiet
        for (int i = 0; i < quietSpace; i++) {
            data.put(emptyLine);
        }
        for (int i = 0; i < borderWidth; i++) {
            data.put(borderLine);
        }

        int[] colors = data.array();
        Bitmap bitmap = Bitmap.createBitmap(colors, totalWidth, totalHeight, Bitmap.Config.ARGB_8888);
        return bitmap;
    }
}
