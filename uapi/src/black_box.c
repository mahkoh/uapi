void *uapi_black_box(void *ptr) {
    asm volatile("" : : "g"(&ptr) : "memory");
    return ptr;
}