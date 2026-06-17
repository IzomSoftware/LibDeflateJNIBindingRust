package net.izom.libdeflater;

import java.nio.charset.StandardCharsets;

public class Main {
    public static void main(String[] args) {
        Binding binding = new Binding();

        byte[] bytes = "asklkfa koapo kfsakfsajiopqfoifw oiqjofiwajiof ka fnaf kaskfasklfklaskjlfajksfkjlasljfkajlsjasjkfjk aasjkjkasf  jakjkfsajklfas jkakja  ajasj kfakjs fjkas jka jkajk sf"
                .getBytes(StandardCharsets.UTF_8);
        byte[] res = binding.compressBytes(bytes);

        System.out.println(String.format("bytes=%s, res=%s", bytes.length, res.length));
    }
}