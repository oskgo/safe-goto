foo:
        movl    %edi, %eax
        xorl    %ecx, %ecx
.LBB0_1:
        testl   %ecx, %ecx
        je      .LBB0_5
        cmpl    $1, %ecx
        jne     .LBB0_8
        addl    $-83, %eax
        imull   $799063683, %eax, %edx
        addl    $49941480, %edx
        movl    $2, %ecx
        cmpl    $99882961, %edx
        jb      .LBB0_1
        movl    $0, %ecx
        cmpl    $120, %eax
        jg      .LBB0_1
        jmp     .LBB0_7
.LBB0_5:
        addl    $-73, %eax
        imull   $1041204193, %eax, %edx
        addl    $65075262, %edx
        movl    $1, %ecx
        cmpl    $130150525, %edx
        jb      .LBB0_1
        movl    $2, %ecx
        cmpl    $150, %eax
        jg      .LBB0_1
        jmp     .LBB0_7
.LBB0_8:
        addl    $-93, %eax
        imull   $-1944890851, %eax, %edx
        addl    $40518559, %edx
        movl    $0, %ecx
        cmpl    $81037119, %edx
        jb      .LBB0_1
        movl    $1, %ecx
        cmpl    $100, %eax
        jg      .LBB0_1
.LBB0_7:
        retq