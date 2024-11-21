import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakeTest } from "../target/types/stake_test";
import { Keypair } from "@solana/web3.js";
import * as fs from 'fs';

describe("stake-test", () => {
  // 配置 anchor provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 从 wallet.json 读取密钥
  const rawdata = fs.readFileSync('./wallet.json', 'utf-8');
  const keypairData = JSON.parse(rawdata);
  const wallet = Keypair.fromSecretKey(
    Uint8Array.from(keypairData)
  );

  const program = anchor.workspace.StakeTest as Program<StakeTest>;

  it("Is initialized!", async () => {
    // 在这里添加您的测试代码
  });
});
