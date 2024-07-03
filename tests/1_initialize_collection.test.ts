import { RnsdidCore } from "../target/types/rnsdid_core";

import {
    Program,
    web3,
    workspace,
    setProvider,
    AnchorProvider,
    BN,
  } from '@project-serum/anchor'

import {
  getUserAssociatedTokenAccount,

  findNonTransferableProject,
  getCollectionVaultAddress,
  getCollectionMintAddress,

  getCollectionMetadataAddress,
  getCollectionMasterEditionAddress,

  getCollectionVaultAccount,
  getTokenWallet,
  createAssociatedTokenAccountInstruction
} from './utils/utils'




import { ADMIN_WALLET, TOKEN_METADATA_PROGRAM_ID, TOKEN_PROGRAM_ID, USER_WALLET } from "./utils/constants";

const { ComputeBudgetProgram, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } = web3

describe("initialize", () => {

    const provider = AnchorProvider.env()
    setProvider(provider)
    const program = workspace.RnsdidCore as Program<RnsdidCore>;

    it("Is initialized!", async () => {

    const nonTransferableProject = await findNonTransferableProject();
    const nonTransferableProjectMint = await getCollectionMintAddress();
    const nonTransferableProjectVault = await getCollectionVaultAddress();
    const nonTransferableProjectMetadata = await getCollectionMetadataAddress(nonTransferableProjectMint);
    const nonTransferableProjectMasterEdition = await getCollectionMasterEditionAddress(nonTransferableProjectMint);
    const collectionVaultAccount = await getCollectionVaultAccount();

    const userTokenAccountAddress = await getUserAssociatedTokenAccount(USER_WALLET.publicKey, nonTransferableProjectMint);


    const transaction = new web3.Transaction()

    const ix_1 = ComputeBudgetProgram.setComputeUnitLimit({
        units: 1_000_000, // 请求的计算单元数量，可以根据需要调整
      });

    const domain = "https://dev-api-1.rns.id/"
    // const domain = "https://api.rns.id/"
    await program.methods
        .initializeCollection({
            name: "Legal DID",
            symbol: 'LDID',
            uri: `${domain}api/v2/portal/identity/collection/metadata/`,
            baseUri: `${domain}api/v2/portal/identity/nft/`
        })
        .accounts({

            authority: ADMIN_WALLET.publicKey,

            nonTransferableProject: nonTransferableProject,
            nonTransferableProjectMint: nonTransferableProjectMint,
            nonTransferableProjectVault: nonTransferableProjectVault,
            nonTransferableProjectMetadata: nonTransferableProjectMetadata,
            nonTransferableProjectMasterEdition: nonTransferableProjectMasterEdition,

            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: web3.SystemProgram.programId,
            rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([
            ADMIN_WALLET,
        ])
        .preInstructions([ix_1])
        .rpc();

        // console.log("Your txhash:", txhash);

    });
});
