package net.izom.libdeflater;

public class Binding {
    public final Binding instance;
    public Binding() {
        System.load("");
        instance = this;
    }
    public native byte[] compressBytes(byte[] bytes);
    public native byte[] decompressBytes(byte[] bytes);
}
