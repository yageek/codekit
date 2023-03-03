package net.yageek.codekit;

import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.Vector;

public class CodeDescriptor {

    public final CodeOptions options;
    final byte[] bars;

    protected CodeDescriptor(CodeOptions options, byte[] bars) {
        this.options = options;
        this.bars = bars;
    }

}
