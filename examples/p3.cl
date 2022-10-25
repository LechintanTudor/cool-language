write("Input base:");
base: i32 = read();

write("Input exponent:");
exponent: i32 = read();

accumulator := 1;

for i in range(accumulator) {
    accumulator *= base;
}

write("Result: ");
write(accumulator);
