# Python 位元組碼格式介紹

## 概述

Python 是一種解釋型語言，但其內部執行並非直接解讀原始碼，而是先將原始碼編譯成一種稱為「位元組碼」（Bytecode）的中間表示式（Intermediate Representation, IR），再由 Python 虛擬機器（PVM, Python Virtual Machine）執行。這種設計讓 Python 能在保持解釋型語言靈活性的同時，獲得近似編譯型語言的執行效率。

## 編譯流程

Python 程式碼的执行流程如下：

```
原始碼 (.py) → 編譯器 → 位元組碼 (.pyc) → 虛擬機器 → 執行結果
```

當 Python 首次匯入模組時，編譯器會將 `.py` 檔案編譯成 `.pyc` 檔案，儲存在 `__pycache__` 目錄中。`.pyc` 檔案包含了魔數（Magic Number）、時間戳記、以及序列化後的程式碼物件（Code Object）。

## .pyc 檔案格式

`.pyc` 檔案的結構如下：

| 欄位 | 位元組數 | 說明 |
|------|---------|------|
| Magic Number | 4 | 版本識別碼，每個 Python 版本皆不同 |
| Bit Field | 4 | 標誌位元組 |
| Timestamp | 4 | 原始檔案修改時間 |
| File Size | 4 | 原始檔案大小 |
| Code Object | N | 序列化後的程式碼物件 |

可以使用 `marshal` 模組來解析 `.pyc` 檔案：

```python
import marshal
import struct
import time

def read_pyc(path):
    with open(path, 'rb') as f:
        magic = f.read(4)
        bit_field = struct.unpack('<I', f.read(4))[0]
        timestamp = struct.unpack('<I', f.read(4))[0]
        file_size = struct.unpack('<I', f.read(4))[0]
        code_obj = marshal.load(f)
        
        print(f'Magic: {magic.hex()}')
        print(f'Bit Field: {bit_field:032b}')
        print(f'Timestamp: {time.ctime(timestamp)}')
        print(f'File Size: {file_size}')
        return code_obj
```

## Code Object 結構

每個 Python 函式、類別、模組 都會編譯成一個 Code Object（類型為 `code`）。Code Object 包含了執行所需的所有靜態資訊：

```python
import inspect
import dis

def show_code_info(func):
    code = func.__code__
    print(f'Function: {func.__name__}')
    print(f'Arguments: {code.co_varnames[:code.co_argcount]}')
    print(f'Local variables: {code.co_varnames}')
    print(f'Constants: {code.co_consts}')
    print(f'Names: {code.co_names}')
    print(f'Cell variables: {code.co_cellvars}')
    print(f'Free variables: {code.co_freevars}')
    print(f'Filename: {code.co_filename}')
    print(f'First line: {code.co_firstlineno}')
    print(f'Code size: {len(code.co_code)} bytes')
    print(f'Stack size: {code.co_stacksize}')
    print(f'Flags: {code.co_flags}')
    
def example_func(a, b):
    c = a + b
    d = a * b
    return c, d

show_code_info(example_func)
```

## 位元組碼指令格式

Python 位元組碼採用變長度指令格式，每條指令由以下部分組成：

| フィールド | 大小 | 說明 |
|-----------|------|------|
| Opcode | 1 byte | 操作碼（0-255） |
| Argument | 0-2 bytes | 參數值（取決於 Opcode） |

當 Opcode >= 60 時，需要 2 個位元組的參數，否則只需要 1 個位元組。整體格式如下：

```
[opcode] [arg_low] (if arg >= 60) [arg_high]
```

## 常見 Opcode 分類

### 載入與儲存操作

| Opcode | 名稱 | 說明 |
|--------|------|------|
| LOAD_FAST | 載入局部變數 | 將局部變數壓入堆疊 |
| LOAD_GLOBAL | 載入全域變數 | 將全域變數壓入堆疊 |
| LOAD_CONST | 載入常數 | 將常數壓入堆疊 |
| STORE_FAST | 儲存局部變數 | 將堆疊頂端存入局部變數 |
| STORE_GLOBAL | 儲存全域變數 | 將堆疊頂端存入全域變數 |
| STORE_NAME | 儲存名稱 | 將堆疊頂端存入名稱 |

### 運算操作

| Opcode | 名稱 | 說明 |
|--------|------|------|
| BINARY_ADD | 加法 | 堆疊彈出兩值相加後推入 |
| BINARY_SUBTRACT | 減法 | 堆疊彈出兩值相減後推入 |
| BINARY_MULTIPLY | 乘法 | 堆疊彈出兩值相乘後推入 |
| BINARY_TRUE_DIVIDE | 除法 | 堆疊彈出兩值相除後推入 |
| BINARY_MODULO | 取餘數 | 堆疊彈出兩值取餘數後推入 |
| COMPARE_OP | 比較 | 執行比較運算 |

### 控制流操作

| Opcode | 名稱 | 說明 |
|--------|------|------|
| POP_JUMP_IF_FALSE | 條件跳躍 | 條件為假時跳躍 |
| POP_JUMP_IF_TRUE | 條件跳躍 | 條件為真時跳躍 |
| JUMP_FORWARD | 無條件前進 | 相對跳躍 |
| JUMP_ABSOLUTE | 無條件絕對跳躍 | 絕對跳躍 |
| FOR_ITER | for 迴圈 | 疊代器遍歷 |

### 函式與類別操作

| Opcode | 名稱 | 說明 |
|--------|------|------|
| CALL_FUNCTION | 函式呼叫 | 呼叫函式 |
| CALL_FUNCTION_KW | 關鍵字參數呼叫 | 帶關鍵字參數呼叫 |
| CALL_FUNCTION_EX | 展開呼叫 | 帶展開參數呼叫 |
| RETURN_VALUE | 返回值 | 返回堆疊頂端值 |
| YIELD_VALUE | 產出值 | 產生生成器值 |

### 屬性與索引操作

| Opcode | 名稱 | 說明 |
|--------|------|------|
| LOAD_ATTR | 載入屬性 | 載入物件屬性 |
| STORE_ATTR | 儲存屬性 | 儲存物件屬性 |
| BINARY_SUBSCR | 索引取值 | 取得索引位置的值 |
| STORE_SUBSCR | 索引儲存 | 設定索引位置的值 |

## 使用 dis 模組

Python 標準庫提供 `dis` 模組来反編譯位元組碼：

```python
import dis

def example(a, b):
    c = a + b
    if c > 10:
        return c * 2
    else:
        return c - 1

print('Disassembly:')
dis.dis(example)
```

輸出結果類似：

```
  2           0 LOAD_FAST                0 (a)
              2 LOAD_FAST                1 (b)
              4 BINARY_ADD
              6 STORE_FAST               2 (c)

  3           8 LOAD_FAST                2 (c)
             10 LOAD_CONST               1 (10)
             12 COMPARE_OP               4 (>)
             14 POP_JUMP_IF_FALSE       22 (to 22)

  4          16 LOAD_FAST                2 (c)
             18 LOAD_CONST               2 (2)
             20 BINARY_MULTIPLY
             22 RETURN_VALUE
```

## Bytecode 指令格式詳解

每條 Bytecode 指令的格式可以通過 `dis.get.instructions()` 詳細查看：

```python
import dis

def example():
    x = 1 + 2
    y = x * 3
    return y

print(f'Constants: {example.__code__.co_consts}')
print(f'Names: {example.__code__.co_names}')
print(f'Variables: {example.__code__.co_varnames}')

for instr in dis.get_instructions(example):
    print(f'{instr.offset:4d} {instr.opname:20s} {instr.argrepr}')
```

輸出：

```
Constants: (None, 1, 2, 3)
Names: ()
Variables: ('x', 'y')
   0 LOAD_CONST               1 (1)
   2 LOAD_CONST               2 (2)
   4 BINARY_ADD
   6 STORE_FAST               0 (x)
   8 LOAD_FAST                0 (x)
  10 LOAD_CONST               3 (3)
  12 BINARY_MULTIPLY
  14 STORE_FAST               1 (y)
  16 LOAD_FAST                1 (y)
  18 RETURN_VALUE
```

## Frame 物件與執行

Python 虛擬機器透過 Frame 物件來執行位元組碼。Frame 包含了執行時的完整上下文：

```python
import types

def frame_info():
    import sys
    frame = sys._getframe(0)
    print(f'Function: {frame.f_code.co_name}')
    print(f'Filename: {frame.f_code.co_filename}')
    print(f'Line: {frame.f_lineno}')
    print(f'Locals: {list(frame.f_locals.keys())}')
    print(f'Globals: {list(frame.f_globals.keys())[:5]}...')
    print(f'Stack depth: {len(frame.f_locals.get("__stack__", []))}')
```

Frame 物件的主要屬性：

| 屬性 | 說明 |
|------|------|
| f_code | 程式碼物件 |
| f_locals | 區域變數字典 |
| f_globals | 全域變數字��� |
| f_builtins | 內建函式字典 |
| f_valuestack | 值堆疊指標 |
| f_back | 前一個 Frame |
| f_lineno | 目前行號 |
| f_lasti | 上個指令索引 |

## 堆疊式虛擬機器

Python VM 是基於堆疊的機器，執行時使用以下堆疊：

- **值堆疊（Value Stack）**：存放運算元與運算結果
- **呼叫堆疊（Call Stack）**：存放函式呼叫框架

執行範例：

```python
def stack_demo(a, b):
    c = a + b  # LOAD_FAST, LOAD_FAST, BINARY_ADD, STORE_FAST
    d = c * 2  # LOAD_FAST, LOAD_CONST, BINARY_MULTIPLY, STORE_FAST
    return d   # LOAD_FAST, RETURN_VALUE
```

執行流程：

```
步驟 1: LOAD_FAST a    → 堆疊: [a]
步驟 2: LOAD_FAST b    → 堆疊: [a, b]
步驟 3: BINARY_ADD    → 堆疊: [c]
步驟 4: STORE_FAST c  → 堆疊: []
步驟 5: LOAD_FAST c    → 堆疊: [c]
步驟 6: LOAD_CONST 2  → 堆疊: [c, 2]
步驟 7: BINARY_MULTIPLY → 堆疊: [d]
步驟 8: STORE_FAST d  → 堆疊: []
步驟 9: LOAD_FAST d    → 堆疊: [d]
步驟10: RETURN_VALUE  → 返回 d
```

## 指令與引數對應

每一個 Opcode 都有其對應的引數解釋方式，以下是完整的對應表：

```python
import dis

# 查看所有 Opcode
print('Extended bytecode:')
for name in dir(dis):
    if name.startswith('opmap'):
        print(f'  {name}')
```

主要的引數對應關係：

| Opcode 範圍 | 引數類型 |
|------------|----------|
| 0-60 | 單引數（直接值） |
| 60-90 | 雙引數（擴展） |
| 90-143 | 區域變數索引 |
| 144-223 | 全域名稱索引 |
| 224-255 | 常數索引 |

## 總結

Python 位元組碼是中間隔離的關鍵層級，它：
1. 將原始碼轉換為高效的可執行格式
2. 提供跨平台相容性
3. 支援動態語言特性如eval和exec
4. 透過dis模組可進行除錯與效能分析
5. 是理解Python內部運作的重要基礎

掌握位元組碼格式對於除錯、效能優化、以及理解Python執行模型都非常有幫助。