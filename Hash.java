import java.util.*;
import java.io.*;

public class Hash {
    public static void main(String[] args) {
        System.out.println(args[0]);
        System.out.println(hash(args[0]));
    }

    public static int hash (String s) {
        int h = 7;
        String letters = "acdegilmnoprstuw";
        for(int i = 0; i < s.length(); i++) {
            h = (h * 37 + letters.indexOf(s.charAt(i)));
        }
        return h;
    }

}
