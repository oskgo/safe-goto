int foo(int n) {
    begin:
    goto first;
    first:
    n -= 73;
    if (n % 33 == 0) {
        goto second;
    } else if (n > 150) {
        goto third;
    }
    return n;
    second:
    n -= 83;
    if (n % 43 == 0) {
        goto third;
    } else if (n > 120) {
        goto first;
    }
    return n;
    third:
    n -= 93;
    if (n % 53 == 0) {
        goto first;
    } else if (n > 100) {
        goto second;
    }
    return n;
}
