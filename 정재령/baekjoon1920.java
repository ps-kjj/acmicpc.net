import java.util.HashSet;
import java.util.Scanner;

public class Main {
    
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);    
        // n 입력
        int n = sc.nextInt();
        // 값 저장할 hashSet( 존재하는지만 검사하기 때문에 중복값 삭제해도 문제 없음 )
        HashSet<Integer> set = new HashSet<>();

        // n개의 수를 받아서 HashSet 에 저장하기
        for (int i = 0; i < n; i++) {
            set.add(sc.nextInt());            
        }
        
        // m 입력
        int m = sc.nextInt();

        for (int i = 0; i < m; i++) {
            if(set.contains(sc.nextInt())){
                System.out.println("1");
            }else{
                System.out.println("0");
            }
        }
        
    }
}
