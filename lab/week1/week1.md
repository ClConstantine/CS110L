# Week1

## 编译

`makefile`略微修改

`windows:`使用`mingw`,保证环境和路径正确的情况下使用`mingw32-make`

`Ubuntu22.04`:`make`

## uppercase

### clang-tidy

```
$ clang-tidy 1-uppercase.c
Error while trying to load a compilation database:
Could not auto-detect compilation database for file "1-uppercase.c"
No compilation database found in /home/cons/Desktop/lab/week1 or any parent directory
fixed-compilation-database: Error while opening fixed database: No such file or directory
json-compilation-database: Error while opening JSON database: No such file or directory
Running without flags.
```

no errors

### 