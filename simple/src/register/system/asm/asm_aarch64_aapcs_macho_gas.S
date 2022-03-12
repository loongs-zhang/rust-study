.text
.globl _prefetch
.align 8
_prefetch:
    prfm pldl1keep, [x0]
    ret

.text
.globl _bootstrap_green_task
.align 8
_bootstrap_green_task:
    mov x0, x19  // arg0
    mov x1, x20  // arg1
    mov x30, #0  // clear LR
    ret x21

//fixme 通过BL指令来实现会更简单，具体可参考《嵌入式C语言自我修养》3.2.7
//汇编实现需要遵守ATPCS规范，具体可参考《嵌入式C语言自我修养》3.6.1
//● 子程序间要通过寄存器R0～R3（可记作a0～a3）传递参数，当参数个数大于4时，剩余的参数使用堆栈来传递。
//● 子程序通过R0～R1返回结果。
//● 子程序中使用R4～R11（可记作v1～v8）来保存局部变量。
//● R12作为调用过程中的临时寄存器，一般用来保存函数的栈帧基址，记作FP。
//● R13作为堆栈指针寄存器，一般记作SP。
//● R14作为链接寄存器，用来保存函数调用者的返回地址，记作LR。
//● R15作为程序计数器，总是指向当前正在运行的指令，记作PC。
.text
.globl _swap_registers
.align 8
_swap_registers:
    stp x19, x20, [x0, #0]
    stp x21, x22, [x0, #16]
    stp x23, x24, [x0, #32]
    stp x25, x26, [x0, #48]
    stp x27, x28, [x0, #64]
    stp x29, x30, [x0, #80]

    //把sp寄存器的值传送到x2寄存器
    mov x2, sp
    //把x2里的数据加载到内存中[x1, #96]地址
    str x2, [x0, #96]

    //[x0, #112]将x0的地址加112生成新内存地址
    stp d8,  d9,  [x0, #112]
    stp d10, d11, [x0, #128]
    stp d12, d13, [x0, #144]
    stp d14, d15, [x0, #160]

    ldp x19, x20, [x1, #0]
    ldp x21, x22, [x1, #16]
    ldp x23, x24, [x1, #32]
    ldp x25, x26, [x1, #48]
    ldp x27, x28, [x1, #64]
    ldp x29, x30, [x1, #80]

    //把内存中[x1, #96]中的值作为地址，将地址上的数据保存到x2
    ldr x2, [x1, #96]
    mov sp, x2

    ldp d8,  d9,  [x1, #112]
    ldp d10, d11, [x1, #128]
    ldp d12, d13, [x1, #144]
    ldp d14, d15, [x1, #160]

    ret
