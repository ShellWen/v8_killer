#include <cstdint>
#include <cstdlib>
#include <cstdio>
#include <cinttypes>

extern "C" int v8_killer_instrument(void *);
extern "C" int v8_killer_instrument_module(void *);
static void *seen_context;
static void *seen_isolate;
static void *seen_source;
extern "C" void v8_killer_process(void *context, void *source) {
  seen_context = context;
  seen_source = source;
  *(uintptr_t *)source += 7;
}
extern "C" void v8_killer_process_module(void *isolate, void *source) {
  seen_isolate = isolate;
  seen_source = source;
  *(uintptr_t *)source += 11;
}

// Like V8 MaybeLocal: user-provided constructor, trivial copy and destructor.
struct MaybeLocal {
  uintptr_t value;
  explicit MaybeLocal(uintptr_t value) : value(value) {}
#ifdef __MINGW32__
  // MinGW needs a nontrivial copy to model MSVC's hidden return pointer.
  MaybeLocal(const MaybeLocal &other) : value(other.value) {}
#endif
};
struct Local { void *value; };

#ifdef _MSC_VER
#define NOINLINE __declspec(noinline)
#else
#define NOINLINE __attribute__((noinline))
#endif

static NOINLINE MaybeLocal compile(Local context, uintptr_t *source, size_t argc,
    Local *argv, size_t extensions, Local *objects, int options, int reason) {
  return MaybeLocal((uintptr_t)context.value + *source + argc + (uintptr_t)argv +
                    extensions + (uintptr_t)objects + options + reason);
}
static NOINLINE MaybeLocal compile_internal(Local context, uintptr_t *source, size_t argc,
    Local *argv, size_t extensions, Local *objects, int options, int reason, Local *module) {
  return MaybeLocal(compile(context, source, argc, argv, extensions, objects, options, reason).value +
                    (uintptr_t)module);
}

static NOINLINE MaybeLocal compile_module(void *isolate, uintptr_t *source,
    int options, int reason) {
  return MaybeLocal((uintptr_t)isolate + *source + options + reason);
}

static void print_entry(const char *name, const void *address) {
  auto bytes = static_cast<const unsigned char *>(address);
  std::printf("%s entry:", name);
  for (size_t i = 0; i < 32; ++i) std::printf(" %02x", static_cast<unsigned int>(bytes[i]));
  std::putchar('\n');
}

int main() {
  std::setvbuf(stdout, nullptr, _IONBF, 0);
  uintptr_t source = 10;
  auto volatile internal = compile_internal;
  auto volatile public_compile = compile;
  auto baseline = public_compile(Local{(void *)1}, &source, 2, (Local *)3, 4, (Local *)5, 6, 7);
  std::printf("CompileFunction baseline: source=%" PRIuPTR " expected=10 result=%" PRIuPTR " expected=38\n",
              source, baseline.value);
  if (source != 10 || baseline.value != 38) return 7;
  std::printf("CompileFunctionInternal=%p CompileFunction=%p CompileModule=%p source=%p\n",
              (void *)compile_internal, (void *)compile, (void *)compile_module, (void *)&source);
  print_entry("CompileFunctionInternal original", (void *)compile_internal);
  print_entry("CompileFunction original", (void *)compile);
  print_entry("CompileModule original", (void *)compile_module);
  auto status = v8_killer_instrument((void *)compile_internal);
  std::printf("CompileFunctionInternal hook status=%d\n", status);
  if (status) return 1;
  auto result = internal(Local{(void *)1}, &source, 2, (Local *)3, 4, (Local *)5, 6, 7, (Local *)8);
  std::printf("CompileFunctionInternal: context=%p expected=%p source_ptr=%p expected=%p source=%" PRIuPTR " expected=17 result=%" PRIuPTR " expected=53\n",
              seen_context, (void *)1, seen_source, (void *)&source, source, result.value);
  if (seen_context != (void *)1 || seen_source != &source || source != 17 || result.value != 53) return 2;
  status = v8_killer_instrument((void *)compile);
  std::printf("CompileFunction hook status=%d\n", status);
  if (status) return 3;
  print_entry("CompileFunction patched", (void *)compile);
  source = 10;
  seen_context = nullptr;
  seen_source = nullptr;
  result = public_compile(Local{(void *)1}, &source, 2, (Local *)3, 4, (Local *)5, 6, 7);
  std::printf("CompileFunction: context=%p expected=%p source_ptr=%p expected=%p source=%" PRIuPTR " expected=17 result=%" PRIuPTR " expected=45\n",
              seen_context, (void *)1, seen_source, (void *)&source, source, result.value);
  if (source != 17 || result.value != 45) return 4;
  auto volatile module = compile_module;
  status = v8_killer_instrument_module((void *)compile_module);
  std::printf("CompileModule hook status=%d\n", status);
  if (status) return 5;
  source = 10;
  seen_source = nullptr;
  result = module((void *)9, &source, 6, 7);
  std::printf("CompileModule: isolate=%p expected=%p source_ptr=%p expected=%p source=%" PRIuPTR " expected=21 result=%" PRIuPTR " expected=43\n",
              seen_isolate, (void *)9, seen_source, (void *)&source, source, result.value);
  if (seen_isolate != (void *)9 || seen_source != &source || source != 21 || result.value != 43) return 6;
  std::puts("Native ABI checks passed");
  return 0;
}
