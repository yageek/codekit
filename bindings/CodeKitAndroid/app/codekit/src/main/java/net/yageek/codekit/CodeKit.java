package net.yageek.codekit;

import android.graphics.Bitmap;

import java.io.BufferedWriter;
import java.nio.ByteBuffer;
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

    private static final int WHITE_COLOR = 0xffff0000;
    private static final int BLACK_COLOR = 0xff000000;

    public native static CodeDescriptor makeEAN8(String code, CodeOptions options);

    public static Bitmap convertBitmap(CodeDescriptor descriptor) {

        final int totalBar = descriptor.bars.length;
        final int barcodeHeight = descriptor.options.getCodeHeight();
        final int borderWidth = descriptor.options.getBorderWidth();
        final int quietSpace = descriptor.options.getQuietSpace();

        final int totalHeight = barcodeHeight + 2*quietSpace + 2*borderWidth;
        final int totalWidth = totalBar + 2*quietSpace + 2*borderWidth;

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

        // We add the bar element
        for (int i = 0; i < totalBar; i++){
            byte value = descriptor.bars[i];

            if (value == 1) {
                codeLine.put(BLACK_COLOR);
            } else {
                codeLine.put(WHITE_COLOR);
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
