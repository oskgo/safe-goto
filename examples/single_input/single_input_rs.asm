foo:
        xorl    %edx, %edx
        leaq    .LJTI0_0(%rip), %r8
.LBB0_1:
        movl    %esi, %eax
        movl    %edx, %edx
        movslq  (%r8,%rdx,4), %rcx
        addq    %r8, %rcx
        movl    %edi, %esi
        movl    $1, %edx
        jmpq    *%rcx
.LBB0_7:
        addl    $-73, %eax
        imull   $1041204193, %eax, %ecx
        addl    $65075262, %ecx
        movl    $2, %edx
        movl    %eax, %esi
        cmpl    $130150525, %ecx
        jb      .LBB0_1
        movl    $3, %edx
        movl    %eax, %esi
        cmpl    $150, %eax
        jg      .LBB0_1
        jmp     .LBB0_4
.LBB0_5:
        addl    $-83, %eax
        imull   $799063683, %eax, %ecx
        addl    $49941480, %ecx
        movl    $3, %edx
        movl    %eax, %esi
        cmpl    $99882961, %ecx
        jb      .LBB0_1
        movl    %eax, %esi
        movl    $1, %edx
        cmpl    $120, %eax
        jg      .LBB0_1
        jmp     .LBB0_4
.LBB0_2:
        addl    $-93, %eax
        imull   $-1944890851, %eax, %ecx
        addl    $40518559, %ecx
        movl    %eax, %esi
        movl    $1, %edx
        cmpl    $81037119, %ecx
        jb      .LBB0_1
        movl    $2, %edx
        movl    %eax, %esi
        cmpl    $100, %eax
        jg      .LBB0_1
.LBB0_4:
        retq
.LJTI0_0:
        .long   .LBB0_1-.LJTI0_0
        .long   .LBB0_7-.LJTI0_0
        .long   .LBB0_5-.LJTI0_0
        .long   .LBB0_2-.LJTI0_0