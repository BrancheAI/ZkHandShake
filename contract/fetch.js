import { Program, Provider, Idl } from "@project-serum/anchor";
import { Connection } from "@solana/web3.js";

const PROGRAM_ID = "YourProgramAddressHere";
const proofAccountPublicKey = new PublicKey("ProofAccountPublicKeyHere"); // The public key of the stored proof

async function fetchProof() {
    // Create a connection to the devnet cluster.
    const connection = new Connection("https://devnet.solana.com");

    // Create a provider, which is used to sign transactions.
    const provider = new Provider(connection, wallet, opts);

    // Load the IDL
    const idl: Idl = JSON.parse(fs.readFileSync("./path_to_idl.json", "utf8"));

    // Create the program interface.
    const program = new Program(idl, PROGRAM_ID, provider);

    // Fetch account details.
    const proofRecord = await program.account.proofRecord.fetch(proofAccountPublicKey);

    return proofRecord.proof;
}

fetchProof().then((proof) => {
    console.log("Retrieved proof:", proof);
});
