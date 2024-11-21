# Solana Staking Contract (Learning Version)

这是一个基于 Solana 区块链的质押合约学习项目。该项目展示了如何使用 Anchor 框架实现一个简单的质押机制。

## 功能特性

该智能合约提供以下核心功能：

1. **初始化质押账户** (`initialize`)
   - 创建新的质押账户
   - 设置账户所有者

2. **质押操作** (`stake`)
   - 允许用户质押 lamports
   - 自动更新质押账户余额

3. **取消质押** (`unstake`)
   - 允许用户提取已质押的 lamports
   - 清零质押账户余额

## 项目结构

```
stake-test/
├── programs/              # 智能合约源代码
│   └── stake-test/       # 主要合约代码
├── tests/                 # 测试文件
├── migrations/           # 部署脚本
├── Anchor.toml           # Anchor 配置文件
├── Cargo.toml            # Rust 依赖配置
└── package.json          # Node.js 依赖配置
```

## 技术栈

- Solana 区块链
- Rust 编程语言
- Anchor 框架
- TypeScript (测试)

## 开始使用

### 前置要求

- Rust 和 Cargo
- Solana CLI 工具
- Node.js 和 Yarn
- Anchor 框架

### 安装步骤

1. 克隆项目
```bash
git clone <项目地址>
cd stake-test
```

2. 安装依赖
```bash
yarn install
```

3. 构建项目
```bash
anchor build
```

4. 运行测试
```bash
anchor test
```

### 部署

1. 确保你有一个 Solana 钱包并且有足够的 SOL
2. 更新 `Anchor.toml` 中的配置
3. 运行部署命令：
```bash
anchor deploy
```

## 使用示例

1. 初始化质押账户：
```typescript
await program.methods
  .initialize()
  .accounts({
    stakeAccount: stakeAccount.publicKey,
    user: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([stakeAccount])
  .rpc();
```

2. 质押代币：
```typescript
await program.methods
  .stake(new BN(amountToStake))
  .accounts({
    stakeAccount: stakeAccount.publicKey,
    userTokenAccount: userTokenAccount,
    vault: vault,
    user: wallet.publicKey,
  })
  .rpc();
```

3. 取消质押：
```typescript
await program.methods
  .unstake()
  .accounts({
    stakeAccount: stakeAccount.publicKey,
    userTokenAccount: userTokenAccount,
    vault: vault,
    user: wallet.publicKey,
  })
  .rpc();
```

## 安全考虑

- 所有操作都需要验证用户签名
- 质押和取消质押操作会检查账户所有权
- 包含基本的错误处理机制

## 贡献指南

欢迎提交 Issues 和 Pull Requests 来改进这个学习项目。

## 许可证

[待添加许可证信息]

## 免责声明

这是一个学习项目，不建议在生产环境中使用。
