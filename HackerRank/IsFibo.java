import java.util.*;
import java.io.*;

// Question: https://www.hackerrank.com/challenges/is-fibo

public class IsFibo {
    public static void main(String[] args) {
        Scanner in = new Scanner(System.in);
        int questions = Integer.parseInt(in.nextLine());

        long[] array = new long[questions];

        for (int i = 0; i < questions; i++) {
            array[i] = Long.parseLong(in.nextLine());
        }

        //System.out.println("Our data array: " + Arrays.toString(array));
        String[] answerArray = isFibo(array);
        for (String answer : answerArray) {
            System.out.println(answer);
        }
    }

    public static String[] isFibo(long[] array) {
        long[] sortedIntArray = new long[array.length];// = array;
        System.arraycopy( array, 0, sortedIntArray, 0, array.length );
        String[] answerArray = new String[array.length];

        for(int z=0; z < answerArray.length; z++) {
            answerArray[z] = "IsNotFibo";
        }

        Arrays.sort(sortedIntArray);
        //System.out.println("Our sorted array: " + Arrays.toString(sortedIntArray));
        //System.out.println("Our unsorted array: " + Arrays.toString(array));

        long first_cache = 1;
        long second_cache = 1;
        long answer = 1;

        for (int i = 0; i < sortedIntArray.length; i++) {
            long numToCheck = sortedIntArray[i];

            if (numToCheck == 1) {
                updateAnswer(array, answerArray, numToCheck);
                continue;
            }

            while (answer <= numToCheck) {
                if (answer == numToCheck) {
                    updateAnswer(array, answerArray, numToCheck);
                    break;
                } else {
                    first_cache = second_cache;
                    second_cache = answer;
                }
                answer = first_cache + second_cache;
            }
        }
        return answerArray;
    }

    public static void updateAnswer(long[] array, String[] answer, long element) {
        for (int i = 0; i < array.length; i++) {
            if (array[i] == element) {
                answer[i] = "IsFibo";
            }
        }
    }
}
