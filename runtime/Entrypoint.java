package me.gulya.rust_dalvik_vm;

import java.io.OutputStream;
import java.io.PrintStream;

class Entrypoint {
    public static native void writeStdout(int i);

    public static void configureEnvironment() {
        System.setOut(new PrintStream(new OutputStream() {
            @Override
            public void write(int i) {
                writeStdout(i);
            }
        }));
    }
}