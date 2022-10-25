write("Input a number:");
n: i32 = read();

write("Started countdown");

while n > 0 {
    write(n);
    n -= 1;
}

write("Countdown finished");
