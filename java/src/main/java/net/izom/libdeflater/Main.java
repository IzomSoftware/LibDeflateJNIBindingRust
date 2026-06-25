package net.izom.libdeflater;

import java.nio.charset.StandardCharsets;

public class Main {
    public static void main(String[] args) {
        Binding binding = new Binding();

        byte[] bytes = "asklkfa koapo kfsakfsajiopqfoifw oiqjofiwajiof ka fnaf kaskfasklfklaskjlfajksfkjlasljfkajlsjasjkfjk aasjkjkasf  jakjkfsajklfas jkakja  ajasj kfakjs fjkas jka jkajk sf"
                .getBytes(StandardCharsets.UTF_8);
        System.out.println("============================================================================");
        System.out.writeBytes(bytes);
        System.out.println();
        byte[] res = binding.compressBytes(bytes);
        System.out.println("============================================================================");
        System.out.writeBytes(res);
        System.out.println();
        byte[] res2 = binding.decompressBytes(res);
        System.out.println("============================================================================");
        System.out.println();
        System.out.writeBytes(res2);
        System.out.println();
    }
}