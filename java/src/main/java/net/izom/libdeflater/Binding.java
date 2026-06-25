package net.izom.libdeflater;

public class Binding {
    static {
        System.load("/home/realstresser/Desktop/Data/Projects/IzomDotNet/software/github/rust/LibDeflateJNIBindingRust/java/src/main/resources/linux-x86-64/libdeflater_jni_binding.so");
    }
    public final Binding instance;
    public Binding() {
        instance = this;
    }

    public native byte[] compressBytes(byte[] bytes);

    public native byte[] decompressBytes(byte[] bytes);
}
