# Advent of Code 2020

Solutions to the Advent of Code 2020 Challenge in Rust

---

## Day 01

### Notes

This is my first (non-trivial) Rust program, and it sure looks like it. I haven't taken the time to go through and read the Rust documentation thoroughly, so this was thrown together with lots of borrowed code from Stack Overflow and the Rust forums.

#### Part 1

It reads in the ints from the file, sorts them, and loops to find if the complement of a number N (2020 - N) exists in the integer vector. If a pair is found, they are printed, as well as their product.

#### Part 2

I essentially did the same thing as part 1, except I used a nested for-loop.

### Solution, Part 1

`757 x 1263 = 956091`

### Solution, Part 2

`1568 x 211 x 241 = 79734368`

---

## Day 02

### Notes

I did a lot of cleanup of the program. Last year, I made individual Python programs/scripts for every day of the challenge. This year I'd like to go for a bit cleaner of a solution that allows for a little more code reuse. With that in mind, I added a system for selecting the day and part of the challenge with command line arguments. Each new day will be done in a separate module. The challenge for day 2 wasn't very hard. It was more about learning the basics of Rust. I'm getting there, but it definitely has a steep learning curve and likes to throw your errors at you. It's quite different from 2019's Python solutions.

### Solution, Part 1

`564`

### Solution, Part 2

`325`

---

## Day 03

### Notes

The first part was a simple character check, increasing the index by 3 mod 31 every line. The second part was essentially the same as the first, but just adding some functionality to make it a little more generic.

All-in-all, this was a pretty easy challenge day. I did it without anything more than a few small errors and I didn't need to look anything up, either. I'm getting more comfortable with Rust.

### Solution, Part 1

`286`

### Solution, Part 2

`3638606400`

---

## Day 04

### Notes

This challenge was good practice for parsing and using Options. The `parse_passports()` function is large and could use refactoring, but I will save that work until later so that I can focus on getting through the rest of the days of the challenge. I'm sure I could make the passport checking code for part 2 more compact, but I decided to write it using the knowledge of Rust I have at hand.

### Solution, Part 1

`222`

### Solution, Part 2

`140`

---

## Day 05

### Notes

The conceit of this challenge is that the seat descriptions are essentially just binary. 'B' and 'R' translate to 1 and 'F' and 'L' translate to 0. From there it's a simple matter of translating the strings to ints and finding the desired values. I could have done a binary search to speed up part 2, but ended up implementing the simplest system first.

### Solution, Part 1

`896`

### Solution, Part 2

`659`

---

## Day 06

### Notes

This is a simple matter of parsing characters and processing them in groups. By translating them to set bits in an int, the groups can be processed differently for both groups. After the parsing, it's a simple matter of bitwise logic and counting bits.

### Solution, Part 1

`6416`

### Solution, Part 2

`3050`

---

## Day 07

### Notes

I was really struggling with this one at first. I was tyring to avoid recursion for no reason in particular. But after bashing my head against a wall for a while I came to my senses and realized the recursive solution was so much easier.

### Solution, Part 1

`238`

### Solution, Part 2

`82930`

---

## Day 08

### Notes

Part 1 took a while, but it was largely due to typos and running into a few new syntactic rules with Rust. Part 2 was largely similar to part 1. I essentially bookmark the state of the program and registers at the instant of the swap. If the swap doesn't cause the program to terminate properly, I go back and perform the swap later. I keep doing this until the proper swap is found.

### Solution, Part 1

`1949`

### Solution, Part 2

`2092`

---

## Day 09

### Notes

These were both pretty simple. It was essentially just a matter of keeping a running buffer which was variable in size in the second case.

### Solution, Part 1

`731031916`

### Solution, Part 2

`93396727`

---

## Day 10

### Notes

Part 1 was simply a matter of sorting the vector and counting the differences. The trick to part 2 was realizing that the number of ways to get to a certain number was the sum of the number of ways to get to the "accessible" numbers below it. Successively generate these values all the way up and you get the answer in O(n) time, despite the fact that there are trillions of permutations.

### Solution, Part 1

`2080`

### Solution, Part 2

`6908379398144`

---

## Day 11

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 12

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 13

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 14

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 15

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 16

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 17

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 18

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 19

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 20

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 21

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 22

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 23

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 24

### Notes

### Solution, Part 1

### Solution, Part 2

---

## Day 25

### Notes

### Solution, Part 1

### Solution, Part 2
