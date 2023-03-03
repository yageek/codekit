package net.yageek.codekit;

import java.nio.ByteBuffer;
import java.util.ArrayList;
import java.util.Vector;

public class CodeDescriptor {

    public final CodeOptions options;
    private final ByteBuffer bars;

    public CodeDescriptor(CodeOptions options, ByteBuffer bars) {
        this.options = options;
        this.bars = bars;
    }
}
