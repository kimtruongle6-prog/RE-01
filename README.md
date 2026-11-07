# RE01 — Comparative Binary Analysis: C++, Rust and Go

## 1. Overview

RE01 is a reverse engineering lab focused on comparing compiled binaries generated from equivalent source code written in C++, Rust, and Go.

The project explores how different programming languages and compiler configurations affect binary size, generated assembly, and decompiled pseudocode.

The compiled binaries are analyzed using Ghidra to observe differences between Debug and Optimized builds.

## 2. Objectives

- Build equivalent programs using C++, Rust, and Go.
- Generate Debug and Optimized binary variants.
- Compare binary sizes across languages and build configurations.
- Examine assembly instructions and decompiled pseudocode using Ghidra.
- Identify compiler optimization techniques, including loop transformations and SIMD vectorization.
- Document analysis results with screenshots and technical findings.
## 3. Project Structure

The RE01 project contains source code, compiled binaries, and analysis evidence organized into separate directories.

```text
RE01/
├── cpp/
│   └── main.cpp
├── rust/
│   └── main.rs
├── go/
│   └── main.go
├── results/
│   ├── cpp_debug.exe
│   ├── cpp_opt.exe
│   ├── rust_debug.exe
│   ├── rust_opt.exe
│   ├── go_debug.exe
│   └── go_opt.exe
└── evidence/
    ├── RE01_CPP_Optimized_Assembly.png
    ├── RE01_Go_Debug_Assembly.png
    ├── RE01_Go_Optimized_Assembly.png
    ├── RE01_Go_Optimized_Main_Call.png
    ├── RE01_Rust_Optimized_Assembly.png
    └── RE01_Rust_Optimized_SIMD_Assembly.png
```
> **Note on Binaries:** Compiled executables (`results/` and `*.exe`) are excluded from version control via `.gitignore` to maintain repository hygiene. Binaries must be compiled locally following the reproduction steps below.
## 4. Development Environment

The RE01 lab was conducted in the following development environment:

| Component | Version / Configuration |
|---|---|
| Operating System | Windows |
| IDE | JetBrains CLion 2026.2.3.1 |
| Terminal | CLion Integrated Terminal (PowerShell) |
| C++ Compiler | GCC 15.2.0 (MinGW-w64) |
| Rust Compiler | rustc 1.99.1 |
| Rust Toolchain | stable-x86_64-pc-windows-gnu |
| Go Compiler | Go 1.27.3 |
| Binary Analysis Tool | Ghidra |

All build commands were executed through the integrated terminal in CLion.

The compiled binaries were saved in the `results/` directory for subsequent analysis.
## 5. Build Instructions

All build commands were executed using PowerShell in the CLion Integrated Terminal.

Run the commands from the root directory of the RE01 project.

### 5.1. C++ Build

**Debug:**
```powershell
g++ -O0 -g -fno-inline -mconsole .\cpp\main.cpp -o .\results\cpp_debug.exe
```

**Optimized:**
```powershell
g++ -O2 -mconsole .\cpp\main.cpp -o .\results\cpp_opt.exe
```

### 5.2. Rust Build

**Debug:**
```powershell
rustup run stable-x86_64-pc-windows-gnu rustc -C opt-level=0 -C debuginfo=2 .\rust\main.rs -o .\results\rust_debug.exe
```

**Optimized:**
```powershell
rustup run stable-x86_64-pc-windows-gnu rustc -C opt-level=2 .\rust\main.rs -o .\results\rust_opt.exe
```

### 5.3. Go Build

**Debug:**
```powershell
go build -gcflags="all=-N -l" -o .\results\go_debug.exe .\go\main.go
```

**Optimized:**
```powershell
go build -o .\results\go_opt.exe .\go\main.go
```

### 5.4. Verify Build Outputs

After building, verify the generated binaries:

```powershell
Get-ChildItem .\results\
```

The `results/` directory should contain six binaries:

- `cpp_debug.exe`
- `cpp_opt.exe`
- `rust_debug.exe`
- `rust_opt.exe`
- `go_debug.exe`
- `go_opt.exe`

## 6. Binary Size Comparison

The six compiled binaries were compared based on their file sizes.

| Language | Debug Size (bytes) | Optimized Size (bytes) |
|---|---:|---:|
| C++ | 62,156 | 46,670 |
| Rust | 5,092,029 | 5,085,469 |
| Go | 2,441,216 | 2,494,464 |

### Observations

- **C++:** The Optimized binary is smaller than the Debug binary.
- **Rust:** The Optimized binary is slightly smaller than the Debug binary.
- **Go:** The Optimized binary is slightly larger than the Debug binary.

### Analysis

Binary size is affected by compiler behavior, runtime components, metadata, debug information, and build configuration.

A smaller binary does not necessarily mean better runtime performance.

Further analysis of assembly instructions and decompiled pseudocode is required to understand the differences between Debug and Optimized builds.
## 7. Ghidra Analysis

Ghidra was used to perform static analysis of the six compiled binaries.

The analysis focused on identifying functions, examining decompiled pseudocode, and inspecting assembly instructions to understand compiler behavior.

### Analysis Workflow

1. Import the compiled binaries from the `results/` directory into Ghidra.
2. Run the initial analysis using Ghidra's auto-analysis.
3. Locate the target functions, particularly `sum_to_n` and `main`.
4. Examine decompiled pseudocode to understand the program logic.
5. Inspect assembly instructions to verify the actual implementation.
6. Compare Debug and Optimized binaries to identify compiler transformations.

### Key Findings

| Language | Notable Observations |
|---|---|
| C++ | Optimized code shows changes in arithmetic and loop structure. |
| Rust | Optimized code uses loop unrolling and SSE2 SIMD instructions. |
| Go | Optimized code uses register-based accumulation instead of the stack-based operations observed in the Debug build. |

### Rust SIMD Observation

The Rust Optimized binary contains SSE2 instructions such as `PADDD` and `PSHUFD`.

These instructions demonstrate packed integer operations and data rearrangement using XMM registers.

The SIMD loop exists in the binary, although the input value used in the lab does not execute that loop.

### Important Note

Decompiler output may not fully represent the underlying assembly behavior. Assembly inspection is therefore necessary to verify important findings.
## 8. Evidence

The following screenshots document the assembly-level analysis performed in Ghidra.

| Evidence File | Description |
|---|---|
| [RE01_CPP_Optimized_Assembly.png](evidence/RE01_CPP_Optimized_Assembly.png) | C++ Optimized assembly analysis. |
| [RE01_Go_Debug_Assembly.png](evidence/RE01_Go_Debug_Assembly.png) | Go Debug assembly analysis. |
| [RE01_Go_Optimized_Assembly.png](evidence/RE01_Go_Optimized_Assembly.png) | Go Optimized assembly analysis. |
| [RE01_Go_Optimized_Main_Call.png](evidence/RE01_Go_Optimized_Main_Call.png) | Main function and call to the target function in Go Optimized. |
| [RE01_Rust_Optimized_Assembly.png](evidence/RE01_Rust_Optimized_Assembly.png) | Rust Optimized assembly analysis. |
| [RE01_Rust_Optimized_SIMD_Assembly.png](evidence/RE01_Rust_Optimized_SIMD_Assembly.png) | SSE2 SIMD instructions in Rust Optimized. |

All screenshots are stored in the `evidence/` directory.
## 9. Limitations

The RE01 lab has the following limitations:

1. **Compiler Configuration Differences**

   Debug and Optimized builds use different compiler options across C++, Rust, and Go. Therefore, the results should not be interpreted as a perfectly controlled comparison of compiler optimization levels across all three languages.

2. **Binary Size Does Not Represent Runtime Performance**

   Binary size alone cannot determine execution speed or overall performance. Runtime benchmarking was not included in this lab.

3. **Limited Function Scope**

   The analysis focuses mainly on the target function `sum_to_n` and the `main` function. It does not cover every function or internal component in the generated binaries.

4. **SIMD Execution Was Not Fully Demonstrated**

   The Rust Optimized binary contains SSE2 SIMD instructions. However, with the input value used in this lab (`n = 10`), the SIMD loop is not executed at runtime. The analysis confirms the presence of SIMD code in the binary, not its execution during the test.

5. **Static Analysis Only**

   The main comparison was performed through static analysis in Ghidra. No comprehensive runtime profiling or performance benchmarking was conducted.

These limitations should be considered when interpreting the findings and comparing the generated binaries.
## 10. Conclusion

The RE01 lab successfully generated and analyzed six executable binaries from equivalent programs written in C++, Rust, and Go.

The comparison between Debug and Optimized builds revealed differences in binary size, assembly structure, and compiler optimization behavior.

Key observations include:

- **C++:** The Optimized build produced a smaller binary and showed changes in arithmetic and loop structure.
- **Rust:** The Optimized build demonstrated loop unrolling and SSE2 SIMD instructions in the generated assembly.
- **Go:** The Optimized build showed register-based accumulation compared with the stack-based operations observed in the Debug build.

This lab demonstrates that different programming languages and compiler configurations can produce significantly different binary structures, even when the source-level logic is equivalent.

Ghidra provided useful insights into these differences through decompiled pseudocode and assembly-level inspection.

Overall, RE01 established a practical foundation for understanding compiler behavior and comparing compiled binaries during reverse engineering.
