package net.yageek.codekit;

/**
 * This class hold all the informations
 * to display a code.
 */
public class CodeDescriptor {

    public final CodeOptions options;
    final byte[] bars;

    protected CodeDescriptor(CodeOptions options, byte[] bars) {
        this.options = options;
        this.bars = bars;
    }
}
