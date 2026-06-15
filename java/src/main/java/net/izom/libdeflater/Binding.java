package net.izom.libdeflater;

public class Binding {
    static {
        System.load("");
    }

    public static native void compressBytes(byte[] bytes);

}
