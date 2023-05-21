import java.util.HashSet;
import java.util.Scanner;

public class IUPC_A {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        String str = sc.next();

        char[] chars = str.toCharArray();

        HashSet<Character> set = new HashSet<>();
        for (int i = 0; i < chars.length; i++) {
            set.add(chars[i]);

        }

        if(set.contains('M') && set.contains('O') &&set.contains('B') &&set.contains('I') &&set.contains('S')){
            System.out.println("YES");
        }else{
            System.out.println("NO");
        }

    }
}
