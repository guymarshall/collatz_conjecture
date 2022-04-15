fn main() {
    println!("Hello, world!");
}

/*
package com.marshall.guy;

import java.math.BigInteger;
import java.util.Scanner;

/*
    USE BIGINTEGER INSTEAD
 */

public class Main {

//    public static int collatz(int number) {
    public static BigInteger collatz(BigInteger number) {
//        int counter = 0;
        BigInteger counter = BigInteger.ZERO;
        // if even, divide by 2
        // else times by 3 and add 1
//        while (number > 1) {
//            counter++;
//            if (number % 2 == 0) {
//                number = number / 2;
//            } else {
//                number = (3 * number) + 1;
//            }
//        }
        while (number.compareTo(BigInteger.ONE) > 0) {
            counter = counter.add(BigInteger.ONE);
            if (number.mod(BigInteger.TWO).equals(BigInteger.ZERO)) {
                number = number.divide(BigInteger.TWO);
            } else {
                number = number.multiply(BigInteger.valueOf(3)).add(BigInteger.ONE);
            }
        }

        return counter;
    }

    public static void main(String[] args) {
        System.out.print("Enter a positive integer: ");
        Scanner scanner = new Scanner(System.in);
//        int userInput = scanner.nextInt();
        BigInteger userInput = scanner.nextBigInteger();

//        int maxNumber = 0;
//        int maxSteps = 0;
        BigInteger maxNumber = BigInteger.ZERO;
        BigInteger maxSteps = BigInteger.ZERO;

//        for (int i = 1; i <= userInput; i++) {
//            if (maxSteps < collatz(i)) {
//                maxSteps = collatz(i);
//                maxNumber = i;
//            }
//
////            System.out.printf("%d: %d%n", i, collatz(i));
//        }

        BigInteger counter = BigInteger.ONE;
        for (; counter.compareTo(userInput) <= 0; counter = counter.add(BigInteger.ONE)) {
            if (maxSteps.compareTo(collatz(counter)) < 0) {
                maxSteps = collatz(counter);
                maxNumber = counter;
            }
        }

        System.out.printf("%d had the maximum number of steps at %d", maxNumber, maxSteps);
    }
}
*/