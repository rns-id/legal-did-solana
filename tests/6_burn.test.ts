import { RnsdidCore } from '../target/types/rnsdid_core'

import {
  Program,
  web3,
  workspace,
  setProvider,
  AnchorProvider,
  BN,
} from '@project-serum/anchor'

import {

  createAssociatedTokenAccountInstruction,



  getOwnershipAccountBump,
  getOwnershipAccountAddress,

  findNonTransferableProject,
  getCollectionMetadataAddress,

  getCollectionMintAddress,
  getCollectionMasterEditionAddress,

  getUserAssociatedTokenAccount,
  getNonTransferableNftMintAddress,

  getTokenAccountBalance,
  getTokenAccountDetails,

  findNonTransferableUserStatus,
  findFreezeAuthority,
  findNonTransferableNftStatus,
  findNonTransferableRnsIdtatus

} from './utils/utils'


import {
     ADMIN_WALLET,
     USER_WALLET,
     TOKEN_METADATA_PROGRAM_ID,
     TOKEN_PROGRAM_ID,
     rnsId,
     tokenIndex } from "./utils/constants";
import { MintLayout, createInitializeMintInstruction } from '@solana/spl-token';
import { assert } from 'chai';
import { PublicKey } from '@solana/web3.js';

const { Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } = web3

describe("burn", () => {



  const provider = AnchorProvider.env();
  setProvider(provider)
  const program = workspace.RnsdidCore as Program<RnsdidCore>;

  it("nft burned successed !", async () => {

    const userPubkey = USER_WALLET.publicKey;

    const collectionAddress = await findNonTransferableProject();

    const collectionMintAddress = await getCollectionMintAddress();
    const collectionMetadataAddress = await getCollectionMetadataAddress(collectionMintAddress);

    const nonTransferableNftMint = await getNonTransferableNftMintAddress(rnsId, tokenIndex);
    const userTokenAccount = await getUserAssociatedTokenAccount(userPubkey, nonTransferableNftMint)

    const details_before = await getTokenAccountDetails(userTokenAccount)
    assert(details_before.amount == BigInt(1), '==');


    const nonTransferableNftMetadata = await getCollectionMetadataAddress(nonTransferableNftMint)
    const nonTransferableNftMasterEdition = await getCollectionMasterEditionAddress(nonTransferableNftMint)
    const nonTransferableNftStatus = await findNonTransferableNftStatus(nonTransferableNftMint);
    const nonTransferableUserStatus = findNonTransferableUserStatus(rnsId, userPubkey);
    const nonTransferableRnsIdStatus = await findNonTransferableRnsIdtatus(rnsId)

    await program.methods
      .burn(rnsId, userPubkey)
      .accounts({
        authority: userPubkey,

        userTokenAccount: userTokenAccount,

        nonTransferableNftMint: nonTransferableNftMint,
        nonTransferableNftMetadata: nonTransferableNftMetadata,
        nonTransferableNftMasterEdition: nonTransferableNftMasterEdition,

        nonTransferableUserStatus: nonTransferableUserStatus,
        nonTransferableNftStatus: nonTransferableNftStatus,
        nonTransferableRnsIdStatus: nonTransferableRnsIdStatus,

        nonTransferableProject: collectionAddress,
        nonTransferableProjectMint: collectionMintAddress,
        nonTransferableProjectMetadata: collectionMetadataAddress,

        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([USER_WALLET])
      .rpc();

    const details_after = await getTokenAccountDetails(userTokenAccount)

    assert(details_after.amount == BigInt(0), '==');

    after(async () => {

        const {isAuthorized, isMinted} = await program.account.userStatusAccount.fetch(nonTransferableUserStatus)
        assert(!isMinted, "did 's is_minted must be false!")

    })
  });
});
