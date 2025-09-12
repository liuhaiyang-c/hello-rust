# Hello Rust

## 基础

### 变量与常量

使用 `let` 声明变量，声明的变量只能被赋值/初始化一次。

```rust
let number: u8;
number = 1; // correct
number = 2; // error
```

结合使用 `mut` 使变量为可变的。

```rust
let mut number = 1;
number = 2; // correct
```

使用 `const` 声明一个常量，必须在声明时进行初始化

```rust
const NUMBER: i32 = 1; // correct
NUMBER = 2; // error
const NUMEBR; // error
const NUMBER = 1; // error
```

#### shadow

Rust 允许同一变量名被多次声明使用，新声明将会遮蔽旧声明

```rust
let x = 6;
let x = 100;
let x = String::from("hello");
// 旧声明不可被使用
```

### 数据类型

#### 标量类型

包括：整型、浮点型、布尔型

##### 整型

| 长度（bit）      | 有符号       | 无符号 |
| ---------------- | ------------ | ------ |
| 8                | i8           | u8     |
| 16               | i16          | u16    |
| 32               | i32(default) | u32    |
| 64               | i64          | u64    |
| 128              | i128         | u128   |
| arch（架构决定） | isize        | usize  |

字面量

| 字面量   | 示例        |
| -------- | ----------- |
| 十进制   | 98_222      |
| 十六进制 | 0xAAA       |
| 八进制   | 0o777       |
| 二进制   | 0b1111_0000 |
| 字节     | b'A'        |

##### 浮点型

| 长度（bit） | 有符号       |
| ----------- | ------------ |
| 32          | f32          |
| 64          | f64(default) |

均为有符号

##### 布尔型

| 长度（bit） | 值      |
| ----------- | ------- |
| 8           | `true`  |
| 8           | `false` |

##### 字符型

使用关键字 `char` 声明

字面量为 `''` 单引号包裹的单个字符，表示一个 Unicode 标量值

占 4 字节

示例：

```rust
let upper_a: char = 'A';
```

#### 复合类型

包括：元组（Tuple）和数组（Array）

##### 元组

固定长度，不同类型元素

示例：

```rust
let tuple: (char, i32, f64) = ('A', 10, 3.14);
// 取用元素
let upper_a = tuple.0;
```

##### 数组

固定长度，相同类型元素

示例：

```rust
let arr: [i32; 3] = [1, 3, 5];
let arr2 = [9;3]; // 内容为：[9, 9, 9]
// 取用元素
let first = arrr[0];
```

### 函数和控制流程

#### 函数

fn func_name(*x: i32*) *-> ret_type* {}

`fn` 声明函数，后接函数名称，常用 `snake_case` 命名风格

`()` 参数列表，使用 `,` 隔开

`->` 表明函数返回类型

`{}` 函数结构体
