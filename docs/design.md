# Design

使用类似于UTXO的模式实现状态转换。

## Leaf

```rust
pub struct Leaf {
    pub owner: Address,
    pub state: Vec<u8>,
    pub index_key: H256,
    pub operator: Index,
    pub leafid: EntityId,
}
```

## Script

可以被作为解锁脚本的内容表示为如下结果：

```rust
pub struct Script<'a> {
    pub version u8,
    pub ty: u8,
    pub code: &'a [u8],
    pub args: &'a [u8],
    pub data: &'a [u8],
}

impl<'a> Script<'a> {
  pub fn address(&self) -> Address {
    let mut hasher = Sha3::new();
        
    let d = &[self.version, self.ty];
    hasher.update(self.code);
    hasher.update(self.args);
    
    hasher.finalize()
  }
}

pub enum ParsedScript {
    Empty,
    Wasm(Vec<u8>, Vec<u8>),
    EVM(Vec<u8>)
}
```

`Script` 结构会被编码为一个Bytes。

## Transaction

```rust
pub struct Transaction {
    pub inputs: Vec<Index>,
    pub unlocker: Vec<u8>,
    pub outputs: Vec<Leaf>
}
```

### Unlocker

用于验证Address是否可用。

Unlocker使用一个bytes编码数据，目前分为三个部分。前两个部分code和args直接哈希计算出来地址，最后一个部分不参与计算。为了验证地址是否正确，执行Unlocker脚本必须返回0的状态码。Unlocker脚本可以访问Leaf的state。

交易验证逻辑：

1. 独立验证每一个Unlocker
2. 对所有的Operator去重，按照顺序执行。

## WASM

使用wasm作为第一版runtime。

### FFI

```rust
extern "C" read_unsigned_transaction_size() -> u32;

extern "C" read_unsigned_transaction(ptr: *const u8);

extern "C" read_leaf_unlocker_size() -> u32;

extern "C" read_leaf_unlocker(ptr: *const u8);
```

## 兼容交易的实现方案

### Token实现

#### 存储

按照如下格式存储数据：

```
data: <asset><amount>
```

#### Script

1. 验证用户的data中的签名与args公钥一致

#### Operation

1. 验证所有的state信息，满足输入 >= 输出的逻辑。

### EVM合约实现

#### 存储

按照如下格式生成EVM的存储：

```
index: H(address, index)
state: u256 * 32
```

#### Script

1. 使用data作为evm输入，执行evm bytecode
2. 执行过程中，使用index读取state
3. 执行的写入后的state，需要对比上输出的变化。

## TxGraph

交易图表示了交易的前后顺序，交易的前后序关系会形成一张依赖图。
