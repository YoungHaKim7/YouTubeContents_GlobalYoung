; ModuleID = 'main.106357af-cgu.0'
source_filename = "main.106357af-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%"core::fmt::Arguments<'_>" = type { { ptr, i64 }, { ptr, i64 }, { ptr, i64 } }

@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17ha827fc4f7acded0aE", [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h684e8f9ddfba3a92E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h227ac09852991115E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h227ac09852991115E" }>, align 8
@alloc17 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc18 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc17, [8 x i8] c"\0C\00\00\00\00\00\00\00" }>, align 8
@alloc4 = private unnamed_addr constant <{}> zeroinitializer, align 8
@alloc31 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483\\library\\core\\src\\fmt\\mod.rs" }>, align 1
@alloc32 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc31, [16 x i8] c"K\00\00\00\00\00\00\00\8C\01\00\00\0D\00\00\00" }>, align 8
@alloc33 = private unnamed_addr constant <{ [9 x i8] }> <{ [9 x i8] c"some text" }>, align 1
@alloc6 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c", " }>, align 1
@alloc7 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc5 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc4, [8 x i8] zeroinitializer, ptr @alloc6, [8 x i8] c"\02\00\00\00\00\00\00\00", ptr @alloc7, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hdc6f183dcdee8a3dE(ptr noundef nonnull %f) unnamed_addr #0 personality ptr @__CxxFrameHandler3 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17ha92b27e4f032c1a7E(ptr noundef nonnull %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !2
  br label %bb4

bb4:                                              ; preds = %start
  ret void

bb2:                                              ; preds = %funclet_bb2
  cleanupret from %cleanuppad unwind to caller

funclet_bb2:                                      ; No predecessors!
  %cleanuppad = cleanuppad within none []
  br label %bb2
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17h7773deb29dfab41aE(ptr noundef nonnull %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 {
start:
  %_9 = alloca ptr, align 8
  %_5 = alloca i64, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %_5)
  call void @llvm.lifetime.start.p0(i64 8, ptr %_9)
  store ptr %main, ptr %_9, align 8
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17h14b9d5955aa99facE(ptr noundef nonnull align 1 %_9, ptr noalias noundef readonly align 8 dereferenceable(24) @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  store i64 %0, ptr %_5, align 8
  %v = load i64, ptr %_5, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_9)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_5)
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h227ac09852991115E"(ptr noalias noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #2 {
start:
  %self = alloca i32, align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr %self)
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hdc6f183dcdee8a3dE(ptr noundef nonnull %_4)
; call <() as std::process::Termination>::report
  %0 = call i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hec9ae3d6bf7a5120E"()
  store i32 %0, ptr %self, align 4
  %_6 = load i32, ptr %self, align 4
  call void @llvm.lifetime.end.p0(i64 4, ptr %self)
  ret i32 %_6
}

; <&T as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h6493d0a25119cd92E"(ptr noalias noundef readonly align 8 dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 0
  %_6.0 = load ptr, ptr %0, align 8, !nonnull !3, !align !4, !noundef !3
  %1 = getelementptr inbounds { ptr, i64 }, ptr %self, i32 0, i32 1
  %_6.1 = load i64, ptr %1, align 8
; call <str as core::fmt::Display>::fmt
  %2 = call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h1d24617662788eeeE"(ptr noalias noundef nonnull readonly align 1 %_6.0, i64 %_6.1, ptr noalias noundef align 8 dereferenceable(64) %f)
  ret i1 %2
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h9dbe9ba903ee610eE(ptr noalias nocapture noundef sret(%"core::fmt::Arguments<'_>") dereferenceable(48) %0, ptr noalias noundef nonnull readonly align 8 %pieces.0, i64 %pieces.1, ptr noalias noundef nonnull readonly align 8 %args.0, i64 %args.1) unnamed_addr #2 {
start:
  %_24 = alloca { ptr, i64 }, align 8
  %_16 = alloca %"core::fmt::Arguments<'_>", align 8
  %_3 = alloca i8, align 1
  call void @llvm.lifetime.start.p0(i64 1, ptr %_3)
  %_4 = icmp ult i64 %pieces.1, %args.1
  br i1 %_4, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_12 = add i64 %args.1, 1
  %_9 = icmp ugt i64 %pieces.1, %_12
  %1 = zext i1 %_9 to i8
  store i8 %1, ptr %_3, align 1
  br label %bb3

bb1:                                              ; preds = %start
  store i8 1, ptr %_3, align 1
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %2 = load i8, ptr %_3, align 1, !range !5, !noundef !3
  %3 = trunc i8 %2 to i1
  br i1 %3, label %bb4, label %bb6

bb6:                                              ; preds = %bb3
  call void @llvm.lifetime.end.p0(i64 1, ptr %_3)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_24)
  store ptr null, ptr %_24, align 8
  %4 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 1
  %5 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 0
  store ptr %pieces.0, ptr %5, align 8
  %6 = getelementptr inbounds { ptr, i64 }, ptr %4, i32 0, i32 1
  store i64 %pieces.1, ptr %6, align 8
  %7 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 0
  %8 = load ptr, ptr %7, align 8, !align !6
  %9 = getelementptr inbounds { ptr, i64 }, ptr %_24, i32 0, i32 1
  %10 = load i64, ptr %9, align 8
  %11 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 0
  store ptr %8, ptr %11, align 8
  %12 = getelementptr inbounds { ptr, i64 }, ptr %0, i32 0, i32 1
  store i64 %10, ptr %12, align 8
  %13 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %0, i32 0, i32 2
  %14 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 0
  store ptr %args.0, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, i64 }, ptr %13, i32 0, i32 1
  store i64 %args.1, ptr %15, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_24)
  ret void

bb4:                                              ; preds = %bb3
  call void @llvm.lifetime.start.p0(i64 48, ptr %_16)
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h9dbe9ba903ee610eE(ptr noalias nocapture noundef sret(%"core::fmt::Arguments<'_>") dereferenceable(48) %_16, ptr noalias noundef nonnull readonly align 8 @alloc18, i64 1, ptr noalias noundef nonnull readonly align 8 @alloc4, i64 0)
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hd2b8fa31b060140fE(ptr noalias nocapture noundef readonly dereferenceable(48) %_16, ptr noalias noundef readonly align 8 dereferenceable(24) @alloc32) #6
  unreachable
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h684e8f9ddfba3a92E"(ptr %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call core::ops::function::FnOnce::call_once
  %1 = call i32 @_ZN4core3ops8function6FnOnce9call_once17hd0abf70ff6c905d0E(ptr noundef nonnull %0)
  ret i32 %1
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17ha92b27e4f032c1a7E(ptr noundef nonnull %_1) unnamed_addr #2 {
start:
  %_2 = alloca {}, align 1
  call void %_1()
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17hd0abf70ff6c905d0E(ptr noundef nonnull %0) unnamed_addr #2 personality ptr @__CxxFrameHandler3 {
start:
  %_2 = alloca {}, align 1
  %_1 = alloca ptr, align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %1 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h227ac09852991115E"(ptr noalias noundef readonly align 8 dereferenceable(8) %_1)
          to label %bb1 unwind label %funclet_bb3

bb3:                                              ; preds = %funclet_bb3
  cleanupret from %cleanuppad unwind to caller

funclet_bb3:                                      ; preds = %start
  %cleanuppad = cleanuppad within none []
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %1
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17ha827fc4f7acded0aE"(ptr %_1) unnamed_addr #2 {
start:
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hec9ae3d6bf7a5120E"() unnamed_addr #2 {
start:
  ret i32 0
}

; main::id
; Function Attrs: uwtable
define internal { ptr, i64 } @_ZN4main2id17haf71ba849d9bb90eE(ptr noalias noundef nonnull readonly align 1 %x.0, i64 %x.1) unnamed_addr #1 {
start:
  %0 = insertvalue { ptr, i64 } undef, ptr %x.0, 0
  %1 = insertvalue { ptr, i64 } %0, i64 %x.1, 1
  ret { ptr, i64 } %1
}

; main::id
; Function Attrs: uwtable
define internal i32 @_ZN4main2id17hf2330cfe58b6ac25E(i32 %x) unnamed_addr #1 {
start:
  ret i32 %x
}

; main::main
; Function Attrs: uwtable
define internal void @_ZN4main4main17h2a68d4d833d7495aE() unnamed_addr #1 {
start:
  %0 = alloca ptr, align 8
  %1 = alloca ptr, align 8
  %2 = alloca ptr, align 8
  %3 = alloca ptr, align 8
  %_15 = alloca { ptr, ptr }, align 8
  %_12 = alloca { ptr, ptr }, align 8
  %_11 = alloca [2 x { ptr, ptr }], align 8
  %_4 = alloca %"core::fmt::Arguments<'_>", align 8
  %string = alloca { ptr, i64 }, align 8
  %int = alloca i32, align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr %int)
; call main::id
  %4 = call i32 @_ZN4main2id17hf2330cfe58b6ac25E(i32 10)
  store i32 %4, ptr %int, align 4
  call void @llvm.lifetime.start.p0(i64 16, ptr %string)
; call main::id
  %5 = call { ptr, i64 } @_ZN4main2id17haf71ba849d9bb90eE(ptr noalias noundef nonnull readonly align 1 @alloc33, i64 9)
  store { ptr, i64 } %5, ptr %string, align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr %_4)
  call void @llvm.lifetime.start.p0(i64 32, ptr %_11)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_12)
  call void @llvm.lifetime.start.p0(i64 8, ptr %3)
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h196ae41d09ba1743E", ptr %3, align 8
  %_21 = load ptr, ptr %3, align 8, !nonnull !3, !noundef !3
  call void @llvm.lifetime.end.p0(i64 8, ptr %3)
  call void @llvm.lifetime.start.p0(i64 8, ptr %2)
  store ptr %int, ptr %2, align 8
  %_23 = load ptr, ptr %2, align 8, !nonnull !3, !align !4, !noundef !3
  call void @llvm.lifetime.end.p0(i64 8, ptr %2)
  store ptr %_23, ptr %_12, align 8
  %6 = getelementptr inbounds { ptr, ptr }, ptr %_12, i32 0, i32 1
  store ptr %_21, ptr %6, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_15)
  call void @llvm.lifetime.start.p0(i64 8, ptr %1)
  store ptr @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h6493d0a25119cd92E", ptr %1, align 8
  %_27 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
  call void @llvm.lifetime.end.p0(i64 8, ptr %1)
  call void @llvm.lifetime.start.p0(i64 8, ptr %0)
  store ptr %string, ptr %0, align 8
  %_29 = load ptr, ptr %0, align 8, !nonnull !3, !align !4, !noundef !3
  call void @llvm.lifetime.end.p0(i64 8, ptr %0)
  store ptr %_29, ptr %_15, align 8
  %7 = getelementptr inbounds { ptr, ptr }, ptr %_15, i32 0, i32 1
  store ptr %_27, ptr %7, align 8
  %8 = getelementptr inbounds { ptr, ptr }, ptr %_12, i32 0, i32 0
  %9 = load ptr, ptr %8, align 8, !nonnull !3, !align !4, !noundef !3
  %10 = getelementptr inbounds { ptr, ptr }, ptr %_12, i32 0, i32 1
  %11 = load ptr, ptr %10, align 8, !nonnull !3, !noundef !3
  %12 = getelementptr inbounds [2 x { ptr, ptr }], ptr %_11, i64 0, i64 0
  %13 = getelementptr inbounds { ptr, ptr }, ptr %12, i32 0, i32 0
  store ptr %9, ptr %13, align 8
  %14 = getelementptr inbounds { ptr, ptr }, ptr %12, i32 0, i32 1
  store ptr %11, ptr %14, align 8
  %15 = getelementptr inbounds { ptr, ptr }, ptr %_15, i32 0, i32 0
  %16 = load ptr, ptr %15, align 8, !nonnull !3, !align !4, !noundef !3
  %17 = getelementptr inbounds { ptr, ptr }, ptr %_15, i32 0, i32 1
  %18 = load ptr, ptr %17, align 8, !nonnull !3, !noundef !3
  %19 = getelementptr inbounds [2 x { ptr, ptr }], ptr %_11, i64 0, i64 1
  %20 = getelementptr inbounds { ptr, ptr }, ptr %19, i32 0, i32 0
  store ptr %16, ptr %20, align 8
  %21 = getelementptr inbounds { ptr, ptr }, ptr %19, i32 0, i32 1
  store ptr %18, ptr %21, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_15)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_12)
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h9dbe9ba903ee610eE(ptr noalias nocapture noundef sret(%"core::fmt::Arguments<'_>") dereferenceable(48) %_4, ptr noalias noundef nonnull readonly align 8 @alloc5, i64 3, ptr noalias noundef nonnull readonly align 8 %_11, i64 2)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17ha94d58fa71d8c3ebE(ptr noalias nocapture noundef readonly dereferenceable(48) %_4)
  call void @llvm.lifetime.end.p0(i64 48, ptr %_4)
  call void @llvm.lifetime.end.p0(i64 32, ptr %_11)
  call void @llvm.lifetime.end.p0(i64 16, ptr %string)
  call void @llvm.lifetime.end.p0(i64 4, ptr %int)
  ret void
}

declare i32 @__CxxFrameHandler3(...) unnamed_addr #3

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h14b9d5955aa99facE(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(24), i64, ptr, i8) unnamed_addr #1

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h1d24617662788eeeE"(ptr noalias noundef nonnull readonly align 1, i64, ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17hd2b8fa31b060140fE(ptr noalias nocapture noundef readonly dereferenceable(48), ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #4

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h196ae41d09ba1743E"(ptr noalias noundef readonly align 4 dereferenceable(4), ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17ha94d58fa71d8c3ebE(ptr noalias nocapture noundef readonly dereferenceable(48)) unnamed_addr #1

define i32 @main(i32 %0, ptr %1) unnamed_addr #3 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h7773deb29dfab41aE(ptr @_ZN4main4main17h2a68d4d833d7495aE, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: argmemonly nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #5

; Function Attrs: argmemonly nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #5

attributes #0 = { noinline uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #3 = { "target-cpu"="x86-64" }
attributes #4 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #5 = { argmemonly nocallback nofree nosync nounwind willreturn }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{i32 1870000}
!3 = !{}
!4 = !{i64 1}
!5 = !{i8 0, i8 2}
!6 = !{i64 8}
