

# RE01 — Comparative Binary Analysis: C++, Rust and Go

## 1. Overview

RE01 is an introductory reverse engineering lab that compares compiled binaries generated from equivalent source programs written in C++, Rust, and Go.

The project explores how programming languages and compiler build configurations affect binary size, generated assembly, and decompiled pseudocode.

Ghidra is used to inspect selected Debug and Optimized binaries, with a focus on understanding differences in generated code and observing compiler optimization patterns.

## 2. Objectives

- Build equivalent programs using C++, Rust, and Go.
- Generate Debug and Optimized binary variants.
- Compare binary sizes across languages and build configurations.
- Examine assembly instructions and decompiled pseudocode using Ghidra.
- Observe compiler optimization patterns, including loop transformations and SIMD instructions.
- Document selected analysis results with screenshots and technical findings.

## 3. Project Structure

The RE01 project contains source code, build outputs, and analysis evidence organized into separate directories.

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

| **Component** | **Version / Configuration** |
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

| **Language** | **Debug Size (bytes)** | **Optimized Size (bytes)** |
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

A smaller binary does not necessarily indicate better runtime performance.

Assembly-level inspection and decompiled pseudocode provide additional information about generated code, but runtime benchmarking would be required to evaluate execution performance.

## 7. Ghidra Analysis

Ghidra was used for static inspection of the compiled binaries, focusing on selected functions and build variants.

The analysis examined decompiled pseudocode and assembly instructions to observe how compiler configurations affect generated code.

### Analysis Workflow

1. Import the compiled binaries from the `results/` directory into Ghidra.
2. Run the initial analysis using Ghidra's auto-analysis.
3. Locate relevant functions, particularly `sum_to_n` / `sumToN` and `main`.
4. Examine decompiled pseudocode to understand the reconstructed program logic.
5. Inspect assembly instructions to verify the underlying implementation.
6. Compare selected Debug and Optimized builds where corresponding evidence is available.

### Key Findings

| **Language** | **Observed Findings** |
|---|---|
| C++ | The Optimized binary shows a transformed arithmetic and loop implementation in the inspected `sum_to_n` function. The available screenshot documents the Optimized build; it does not provide a direct visual Debug-versus-Optimized comparison. |
| Rust | The Optimized binary contains an unrolled accumulation pattern and SSE2 SIMD instructions in the inspected function. |
| Go | The inspected Optimized function uses register-based accumulation. In the Debug build, the assembly shows stack-based operations for the accumulation variables. |

### Go Debug: Assembly and Decompiler Observation

In the Go Debug screenshot, the assembly listing shows stack-based operations associated with the accumulation variables.

However, the decompiled pseudocode displays a loop without the corresponding accumulation statement.

This difference illustrates that decompiler output may not always reconstruct the original program logic completely. For this case, the assembly listing provides the more useful evidence for examining the generated operations.

### Rust SIMD Observation

The Rust Optimized binary contains SSE2 instructions, including:

- `PADDD` — packed integer addition.
- `PSHUFD` — packed integer data rearrangement.
- `MOVDQA` — aligned packed data movement.
- `PXOR` — packed bitwise XOR.

These instructions show that SIMD-related code is present in the generated binary.

The SIMD loop exists in the binary, although the input value used in the lab does not execute that loop at runtime. Therefore, this finding confirms the presence of SIMD instructions in the binary, not their execution during the test.

### Important Note

Decompiler output is a reconstruction of program behavior and may not fully represent the underlying assembly.

Important findings should therefore be checked against the assembly instructions rather than relying on pseudocode alone.

## 8. Evidence

The following screenshots document selected assembly-level analysis performed in Ghidra.

| **Evidence File** | **Description** |
|---|---|
| [RE01_CPP_Optimized_Assembly.png](https://github.com/kimtruongle6-prog/RE-01/blob/main/evidence/RE01_CPP_Optimized_Assembly.png) | C++ Optimized `sum_to_n` function, showing assembly and decompiled pseudocode. |
| [RE01_Go_Debug_Assembly.png](https://github.com/kimtruongle6-prog/RE-01/blob/main/evidence/RE01_Go_Debug_Assembly.png) | Go Debug `main.sumToN` function, showing stack-based assembly operations and decompiler output. |
| [RE01_Go_Optimized_Assembly.png](https://github.com/kimtruongle6-prog/RE-01/blob/main/evidence/RE01_Go_Optimized_Assembly.png) | Go Optimized `main.sumToN` function, showing register-based accumulation and decompiled pseudocode. |
| [RE01_Go_Optimized_Main_Call.png](https://github.com/kimtruongle6-prog/RE-01/blob/main/evidence/RE01_Go_Optimized_Main_Call.png) | Go Optimized `main` function and its call to `main.sumToN(10)`. |
| [RE01_Rust_Optimized_Assembly.png](https://github.com/kimtruongle6-prog/RE-01/blob/main/evidence/RE01_Rust_Optimized_Assembly.png) | Rust Optimized `sum_to_n` function, showing the generated assembly and decompiled pseudocode. |
| [RE01_Rust_Optimized_SIMD_Assembly.png](https://github.com/kimtruongle6-prog/RE-01/blob/main/evidence/RE01_Rust_Optimized_SIMD_Assembly.png) | Rust Optimized assembly containing SSE2 SIMD instructions, including `PADDD` and `PSHUFD`. |

All screenshots are stored in the `evidence/` directory.

## 9. Limitations

The RE01 lab has the following limitations:

1. **Compiler Configuration Differences**

   Debug and Optimized builds use different compiler options across C++, Rust, and Go. Therefore, the results should not be interpreted as a perfectly controlled comparison of compiler optimization levels across all three languages.

2. **Uneven Evidence Coverage**

   The current Ghidra screenshots document selected build variants rather than providing equivalent Debug and Optimized evidence for every language. In particular, the available screenshots do not include C++ Debug or Rust Debug analysis.

3. **Binary Size Does Not Represent Runtime Performance**

   Binary size alone cannot determine execution speed or overall performance. Runtime benchmarking was not included in this lab.

4. **Limited Function Scope**

   The analysis focuses mainly on the target function (`sum_to_n` / `sumToN`) and the `main` function. It does not cover every function or internal component in the generated binaries.

5. **SIMD Execution Was Not Fully Demonstrated**

   The Rust Optimized binary contains SSE2 SIMD instructions. However, with the input value used in this lab (`n = 10`), the SIMD loop is not executed at runtime. The analysis confirms the presence of SIMD code in the binary, not its execution during the test.

6. **Static Analysis Only**

   The main comparison was performed through static analysis in Ghidra. No comprehensive runtime profiling or performance benchmarking was conducted.

These limitations should be considered when interpreting the findings and comparing the generated binaries.

## 10. Conclusion

The RE01 lab generated six executable binaries from equivalent programs written in C++, Rust, and Go, covering Debug and Optimized build configurations.

The binary size comparison revealed differences across languages and build configurations. Selected Ghidra inspections also showed differences in generated assembly and decompiled pseudocode.

The main observations include:

- **C++:** The inspected Optimized function shows a transformed arithmetic and loop implementation.
- **Rust:** The Optimized binary contains an unrolled accumulation pattern and SSE2 SIMD instructions.
- **Go:** The inspected Optimized function uses register-based accumulation, while the Debug assembly shows stack-based operations.

These observations demonstrate that source-level equivalent programs can produce different binary structures depending on the programming language and compiler configuration.

Ghidra provided a practical way to inspect these differences through decompiled pseudocode and assembly-level analysis.

RE01 establishes a foundation for further reverse engineering work, with future labs able to investigate compiler transformations, binary structures, and runtime behavior in greater depth.

