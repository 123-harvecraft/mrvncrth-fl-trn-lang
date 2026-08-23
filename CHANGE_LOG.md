# 07-06-2026

From inside your project directory:

```powershell
trn-pkg build
```

Or to build **and run** in one step:

```powershell
trn-pkg run
```

To compile a **single file**:

```powershell
trnc src/main.tru          # compile → executable
trnc src/main.tru --run    # compile + run
trn-run src/main.tru       # same as above
```

> **Note:** If the commands aren't found yet, add to PATH first:
> ```powershell
> $env:PATH += ";$env:USERPROFILE\.trnlang\bin"
> ```
> Then restart your terminal — it will be permanent.