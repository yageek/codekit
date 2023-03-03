package net.yageek.codekit;

import java.util.ArrayList;
import java.util.Vector;

public class CodeDescriptor {

    public final CodeOptions options;
    private final ArrayList<Boolean> bars;

    CodeDescriptor(CodeOptions options, ArrayList<Boolean> bars) {
        this.options = options;
        this.bars = bars;
    }
}
