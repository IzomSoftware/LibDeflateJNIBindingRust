package net.izom.libdeflater;

public class Binding {
    static {

    }
    public final Binding instance;

    public Binding() {
        instance = this;
    }

    public native byte[] compressBytes(byte[] bytes);

    public native byte[] decompressBytes(byte[] bytes);
}
