failed function body: 
  • CompoundStmt("" src/alloc-override-win.c:456:1)
    • IfStmt("" src/alloc-override-win.c:457:3)
      • BinaryOperator("" src/alloc-override-win.c:457:7)
        +Int<"int">
        • ImplicitCastExpr!("original" src/alloc-override-win.c:457:14)
          +Pointer<"void *" ->Void<"void">>
          • MemberRefExpr("original" src/alloc-override-win.c:457:14)
            +Pointer<"void *" ->Void<"void">>
            • ImplicitCastExpr!("patch" src/alloc-override-win.c:457:7)
              +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
              • DeclRefExpr("patch" src/alloc-override-win.c:457:7)
                +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
        • ParenExpr("" src/alloc-override-win.c:457:26)
          +Pointer<"void *" ->Void<"void">>
          • CStyleCastExpr("" src/alloc-override-win.c:457:26)
            +Pointer<"void *" ->Void<"void">>
            • IntegerLiteral("" src/alloc-override-win.c:457:26)
              +Int<"int">
      • ReturnStmt("" src/alloc-override-win.c:457:32)
        • ImplicitCastExpr!("" src/alloc-override-win.c:457:39)
          +Bool<"_Bool">
          • IntegerLiteral("" src/alloc-override-win.c:457:39)
            +Int<"int">
    • IfStmt("" src/alloc-override-win.c:458:3)
      • BinaryOperator("" src/alloc-override-win.c:458:7)
        +Int<"int">
        • BinaryOperator("" src/alloc-override-win.c:458:7)
          +Int<"int">
          • ImplicitCastExpr!("apply" src/alloc-override-win.c:458:7)
            +UInt<"unsigned int">
            • ImplicitCastExpr!("apply" src/alloc-override-win.c:458:7)
              +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
              • DeclRefExpr("apply" src/alloc-override-win.c:458:7)
                +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
          • ImplicitCastExpr!("PATCH_TARGET_TERM" src/alloc-override-win.c:458:16)
            +UInt<"unsigned int">
            • DeclRefExpr("PATCH_TARGET_TERM" src/alloc-override-win.c:458:16)
              +Int<"int">
        • BinaryOperator("" src/alloc-override-win.c:458:37)
          +Int<"int">
          • ImplicitCastExpr!("target_term" src/alloc-override-win.c:458:44)
            +Pointer<"void *" ->Void<"void">>
            • MemberRefExpr("target_term" src/alloc-override-win.c:458:44)
              +Pointer<"void *" ->Void<"void">>
              • ImplicitCastExpr!("patch" src/alloc-override-win.c:458:37)
                +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
                • DeclRefExpr("patch" src/alloc-override-win.c:458:37)
                  +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
          • ParenExpr("" src/alloc-override-win.c:458:59)
            +Pointer<"void *" ->Void<"void">>
            • CStyleCastExpr("" src/alloc-override-win.c:458:59)
              +Pointer<"void *" ->Void<"void">>
              • IntegerLiteral("" src/alloc-override-win.c:458:59)
                +Int<"int">
      • BinaryOperator("" src/alloc-override-win.c:458:65)
        +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
        • DeclRefExpr("apply" src/alloc-override-win.c:458:65)
          +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
        • ImplicitCastExpr!("PATCH_TARGET" src/alloc-override-win.c:458:73)
          +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
          • DeclRefExpr("PATCH_TARGET" src/alloc-override-win.c:458:73)
            +Int<"int">
    • IfStmt("" src/alloc-override-win.c:459:3)
      • BinaryOperator("" src/alloc-override-win.c:459:7)
        +Int<"int">
        • ImplicitCastExpr!("applied" src/alloc-override-win.c:459:14)
          +UInt<"unsigned int">
          • ImplicitCastExpr!("applied" src/alloc-override-win.c:459:14)
            +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
            • MemberRefExpr("applied" src/alloc-override-win.c:459:14)
              +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
              • ImplicitCastExpr!("patch" src/alloc-override-win.c:459:7)
                +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
                • DeclRefExpr("patch" src/alloc-override-win.c:459:7)
                  +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
        • ImplicitCastExpr!("apply" src/alloc-override-win.c:459:25)
          +UInt<"unsigned int">
          • ImplicitCastExpr!("apply" src/alloc-override-win.c:459:25)
            +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
            • DeclRefExpr("apply" src/alloc-override-win.c:459:25)
              +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
      • ReturnStmt("" src/alloc-override-win.c:459:32)
        • ImplicitCastExpr!("" src/alloc-override-win.c:459:39)
          +Bool<"_Bool">
          • IntegerLiteral("" src/alloc-override-win.c:459:39)
            +Int<"int">
    • IfStmt("" src/alloc-override-win.c:462:3)
      • OpaqueValueExpr!("" builtin definitions)
        +Bool<"_Bool">
      • ReturnStmt("" src/alloc-override-win.c:462:89)
        • ImplicitCastExpr!("" src/alloc-override-win.c:462:96)
          +Bool<"_Bool">
          • IntegerLiteral("" src/alloc-override-win.c:462:96)
            +Int<"int">
    • IfStmt("" src/alloc-override-win.c:463:3)
      • BinaryOperator("" src/alloc-override-win.c:463:7)
        +Int<"int">
        • ImplicitCastExpr!("apply" src/alloc-override-win.c:463:7)
          +UInt<"unsigned int">
          • ImplicitCastExpr!("apply" src/alloc-override-win.c:463:7)
            +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
            • DeclRefExpr("apply" src/alloc-override-win.c:463:7)
              +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
        • ImplicitCastExpr!("PATCH_NONE" src/alloc-override-win.c:463:16)
          +UInt<"unsigned int">
          • DeclRefExpr("PATCH_NONE" src/alloc-override-win.c:463:16)
            +Int<"int">
      • CompoundStmt("" src/alloc-override-win.c:463:28)
      • CompoundStmt("" src/alloc-override-win.c:466:8)
        • DeclStmt("" src/alloc-override-win.c:467:5)
          • VarDecl("target" src/alloc-override-win.c:467:11)
            +Pointer<"void *" ->Void<"void">>
            • ParenExpr("" src/alloc-override-win.c:467:20)
              +Pointer<"void *" ->Void<"void">>
              • ConditionalOperator("" src/alloc-override-win.c:467:21)
                +Pointer<"void *" ->Void<"void">>
                • BinaryOperator("" src/alloc-override-win.c:467:21)
                  +Int<"int">
                  • ImplicitCastExpr!("apply" src/alloc-override-win.c:467:21)
                    +UInt<"unsigned int">
                    • ImplicitCastExpr!("apply" src/alloc-override-win.c:467:21)
                      +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
                      • DeclRefExpr("apply" src/alloc-override-win.c:467:21)
                        +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
                  • ImplicitCastExpr!("PATCH_TARGET" src/alloc-override-win.c:467:30)
                    +UInt<"unsigned int">
                    • DeclRefExpr("PATCH_TARGET" src/alloc-override-win.c:467:30)
                      +Int<"int">
                • ImplicitCastExpr!("target" src/alloc-override-win.c:467:52)
                  +Pointer<"void *" ->Void<"void">>
                  • MemberRefExpr("target" src/alloc-override-win.c:467:52)
                    +Pointer<"void *" ->Void<"void">>
                    • ImplicitCastExpr!("patch" src/alloc-override-win.c:467:45)
                      +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
                      • DeclRefExpr("patch" src/alloc-override-win.c:467:45)
                        +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
                • ImplicitCastExpr!("target_term" src/alloc-override-win.c:467:68)
                  +Pointer<"void *" ->Void<"void">>
                  • MemberRefExpr("target_term" src/alloc-override-win.c:467:68)
                    +Pointer<"void *" ->Void<"void">>
                    • ImplicitCastExpr!("patch" src/alloc-override-win.c:467:61)
                      +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
                      • DeclRefExpr("patch" src/alloc-override-win.c:467:61)
                        +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
        • NullStmt("" src/alloc-override-win.c:468:37)
    • BinaryOperator("" src/alloc-override-win.c:471:3)
      +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
      • MemberRefExpr("applied" src/alloc-override-win.c:471:10)
        +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
        • ImplicitCastExpr!("patch" src/alloc-override-win.c:471:3)
          +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
          • DeclRefExpr("patch" src/alloc-override-win.c:471:3)
            +Pointer<"mi_patch_t *" canon=Pointer<"struct mi_patch_s *" ->Record<"struct mi_patch_s" decl=StructDecl("mi_patch_s" src/alloc-override-win.c:378:16)>> ->Typedef<"mi_patch_t" decl=TypedefDecl("mi_patch_t" src/alloc-override-win.c:385:3)>>
      • ImplicitCastExpr!("apply" src/alloc-override-win.c:471:20)
        +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
        • DeclRefExpr("apply" src/alloc-override-win.c:471:20)
          +Typedef<"patch_apply_t" decl=TypedefDecl("patch_apply_t" src/alloc-override-win.c:376:3)>
    • ReturnStmt("" src/alloc-override-win.c:473:3)
      • ImplicitCastExpr!("" src/alloc-override-win.c:473:10)
        +Bool<"_Bool">
        • IntegerLiteral("" src/alloc-override-win.c:473:10)
          +Int<"int">

