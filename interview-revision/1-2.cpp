#include <stdio.h>

void reverse(char *str) {
    char *end = str;
    char tmp;

    // Make sure it's not a null list
    if (str) {
        // Go to the end till you reach null
        while (*end) {
            ++end;
        }
        // Head one char back so you avoid the null again
        --end;

        // Do the hokey-pokey
        while (str < end) {
            tmp = *str;
            *str++ = *end;
            *end-- = tmp;
        }

    }
}

int main() {
    char foo[] = "this is a test";
    printf("%s\n", foo);
    reverse(foo);
    printf("%s\n", foo);

}
