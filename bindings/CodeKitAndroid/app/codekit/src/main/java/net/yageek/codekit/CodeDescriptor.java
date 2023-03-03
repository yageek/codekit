package net.yageek.codekit;

public class CodeDescriptor {

    public final CodeOptions options;
    final byte[] bars;

    protected CodeDescriptor(CodeOptions options, byte[] bars) {
        this.options = options;
        this.bars = bars;
    }
}
