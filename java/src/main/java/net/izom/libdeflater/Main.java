package net.izom.libdeflater;

import java.nio.charset.StandardCharsets;

public class Main {
    public static void main(String[] args) {
        String txt = "asafgagsgsagag";
        byte[] input = txt.getBytes(StandardCharsets.UTF_8);
        // byte[] result = Binding.compressBytes(input);
        StringBuilder t = new StringBuilder();
        // for (byte b : result) {
        // }
        Binding.compressBytes("test 1233".getBytes());
    }
}
