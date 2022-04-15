fn main() {
    println!("Hello, world!");
}

// public static int collatz(int number) {
//     int counter = 0;
//     if even, divide by 2
//     else times by 3 and add 1
//     while (number > 1) {
//         counter++;
//         if (number % 2 == 0) {
//             number = number / 2;
//         } else {
//             number = (3 * number) + 1;
//         }
//     }

//     return counter;
// }

// public static void main(String[] args) {
//     System.out.print("Enter a positive integer: ");
//     Scanner scanner = new Scanner(System.in);
//     int userInput = scanner.nextInt();

//     int maxNumber = 0;
//     int maxSteps = 0;

//     for (int i = 1; i <= userInput; i++) {
//         if (maxSteps < collatz(i)) {
//             maxSteps = collatz(i);
//             maxNumber = i;
//         }

//         System.out.printf("%d: %d%n", i, collatz(i));
//     }

//     System.out.printf("%d had the maximum number of steps at %d", maxNumber, maxSteps);
// }