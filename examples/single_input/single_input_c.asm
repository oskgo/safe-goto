foo:                                    # @foo
.LBB0_1:                                # %.backedge
        addl    $-73, %edi
        movslq  %edi, %rax
        imulq   $1041204193, %rax, %rcx # imm = 0x3E0F83E1
        movq    %rcx, %rdx
        shrq    $63, %rdx
        sarq    $35, %rcx
        addl    %edx, %ecx
        imull   $33, %ecx, %ecx
        cmpl    %ecx, %eax
        je      .LBB0_5
        cmpl    $150, %edi
        jle     .LBB0_7
.LBB0_3:                                #   in Loop: Header=BB0_1 Depth=1
        addl    $-93, %edi
        movslq  %edi, %rax
        imulq   $1296593901, %rax, %rcx # imm = 0x4D4873ED
        movq    %rcx, %rdx
        shrq    $63, %rdx
        sarq    $36, %rcx
        addl    %edx, %ecx
        imull   $53, %ecx, %ecx
        cmpl    %ecx, %eax
        je      .LBB0_1
        cmpl    $100, %edi
        jle     .LBB0_7
.LBB0_5:                                #   in Loop: Header=BB0_1 Depth=1
        addl    $-83, %edi
        movslq  %edi, %rax
        imulq   $799063683, %rax, %rcx  # imm = 0x2FA0BE83
        movq    %rcx, %rdx
        shrq    $63, %rdx
        sarq    $35, %rcx
        addl    %edx, %ecx
        imull   $43, %ecx, %ecx
        cmpl    %ecx, %eax
        je      .LBB0_3
        cmpl    $120, %edi
        jg      .LBB0_1
.LBB0_7:
        movl    %edi, %eax
        ret