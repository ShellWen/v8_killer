#include <cstdint>
#include <cstdlib>

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

int main() {
  uintptr_t source = 10;
  auto volatile internal = compile_internal;
  auto volatile public_compile = compile;
  if (v8_killer_instrument((void *)compile_internal)) return 1;
  auto result = internal(Local{(void *)1}, &source, 2, (Local *)3, 4, (Local *)5, 6, 7, (Local *)8);
  if (seen_context != (void *)1 || seen_source != &source || source != 17 || result.value != 53) return 2;
  if (v8_killer_instrument((void *)compile)) return 3;
  source = 10;
  result = public_compile(Local{(void *)1}, &source, 2, (Local *)3, 4, (Local *)5, 6, 7);
  if (source != 17 || result.value != 45) return 4;
  auto volatile module = compile_module;
  if (v8_killer_instrument_module((void *)compile_module)) return 5;
  source = 10;
  result = module((void *)9, &source, 6, 7);
  if (seen_isolate != (void *)9 || seen_source != &source || source != 21 || result.value != 43) return 6;
  return 0;
}
