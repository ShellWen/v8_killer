#include "dobby.h"
#include <cstring>
#include <string>
#include <vector>
#ifdef _WIN32
#include <windows.h>
#include <psapi.h>
#else
#include <dlfcn.h>
#ifdef __APPLE__
#include <mach-o/dyld.h>
#else
#include <link.h>
#include <unistd.h>
#endif
#endif

extern "C" void v8_killer_process(void *context, void *source);

static void on_enter(RegisterContext *ctx, const HookEntryInfo *) {
#if defined(_WIN32) && (defined(_M_X64) || defined(__x86_64__))
  v8_killer_process((void *)ctx->general.regs.rdx, (void *)ctx->general.regs.r8);
#elif defined(__x86_64__)
  v8_killer_process((void *)ctx->general.regs.rdi, (void *)ctx->general.regs.rsi);
#elif !defined(_WIN32) && (defined(__aarch64__) || defined(__arm64__))
  v8_killer_process((void *)ctx->general.regs.x0, (void *)ctx->general.regs.x1);
#else
#error Unsupported V8 entry ABI
#endif
}

extern "C" int v8_killer_instrument(void *address) {
  return DobbyInstrument(address, on_enter);
}

extern "C" void *v8_killer_symbol(const char *name) {
#ifdef _WIN32
  DWORD needed = 0;
  std::vector<HMODULE> modules(128);
  for (;;) {
    if (!EnumProcessModules(GetCurrentProcess(), modules.data(),
                            (DWORD)(modules.size() * sizeof(HMODULE)), &needed))
      return nullptr;
    if (needed <= modules.size() * sizeof(HMODULE)) break;
    modules.resize(needed / sizeof(HMODULE));
  }
  for (size_t i = 0; i < needed / sizeof(HMODULE); ++i)
    if (auto symbol = GetProcAddress(modules[i], name)) return (void *)symbol;
  return nullptr;
#else
  return dlsym(RTLD_DEFAULT, name);
#endif
}

extern "C" void *v8_killer_module(const char *name) {
#ifdef _WIN32
  auto module = GetModuleHandleA(name);
  return module ? module : LoadLibraryA(name);
#elif defined(__APPLE__)
  dlopen(name, RTLD_NOW | RTLD_GLOBAL);
  for (uint32_t i = 0; i < _dyld_image_count(); ++i) {
    const char *path = _dyld_get_image_name(i);
    const char *base = strrchr(path, '/');
    if (!strcmp(path, name) || (base && !strcmp(base + 1, name)))
      return (void *)_dyld_get_image_header(i);
  }
  return nullptr;
#else
  dlopen(name, RTLD_NOW | RTLD_GLOBAL);
  struct Search { const char *name; void *base; } search{name, nullptr};
  dl_iterate_phdr([](dl_phdr_info *info, size_t, void *data) {
    auto &s = *(Search *)data;
    const char *path = info->dlpi_name;
    std::string executable;
    if (!*path) {
      char buffer[4096];
      auto length = readlink("/proc/self/exe", buffer, sizeof(buffer));
      if (length < 0) return 0;
      executable.assign(buffer, length);
      path = executable.c_str();
    }
    const char *base = strrchr(path, '/');
    if (strcmp(path, s.name) && (!base || strcmp(base + 1, s.name))) return 0;
    for (size_t i = 0; i < info->dlpi_phnum; ++i) {
      if (info->dlpi_phdr[i].p_type == PT_LOAD) {
        s.base = (void *)(info->dlpi_addr + info->dlpi_phdr[i].p_vaddr);
        return 1;
      }
    }
    return 0;
  }, &search);
  return search.base;
#endif
}
