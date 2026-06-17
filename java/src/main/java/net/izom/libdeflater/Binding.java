package net.izom.libdeflater;

public class Binding {
    static {
        System.load("");
    }

    public static native byte[] compressBytes(byte[] bytes);

}
