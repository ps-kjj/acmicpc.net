import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.util.LinkedList;
import java.util.ListIterator;
import java.util.Stack;
import java.util.StringTokenizer;

public class boj1406 {
    public static void main(String[] args) throws IOException {
        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));

        Stack<String> leftStack = new Stack<>();
        Stack<String> rightStack = new Stack<>();

        String[] str = br.readLine().split("");
        for (String s : str) {
            leftStack.push(s);
        }
        int M = Integer.parseInt(br.readLine());
        StringTokenizer st;
        for (int i = 0; i < M; i++) {
            st = new StringTokenizer(br.readLine());
            switch (st.nextToken()) {
                case "L":
                    if (!leftStack.isEmpty()) {
                        rightStack.push(leftStack.pop());
                    }
                    break;
                case "D":
                    if (!rightStack.isEmpty()) {
                        leftStack.push(rightStack.pop());
                    }
                    break;
                case "B":
                    if (!leftStack.isEmpty()) {
                        leftStack.pop();
                    }
                    break;
                case "P":
                    leftStack.push(st.nextToken());
                    break;
            }
        }
        StringBuilder sb = new StringBuilder();
        while(!leftStack.isEmpty()){
            rightStack.push(leftStack.pop());
        }

        while(!rightStack.isEmpty()){
            sb.append(rightStack.pop());
        }
        System.out.println(sb);
    }
}