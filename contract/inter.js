const solanaWeb3 = require('@solana/web3.js');

// Solana connection setup
const connection = new solanaWeb3.Connection(solanaWeb3.clusterApiUrl('devnet'));
const wallet = new solanaWeb3.Keypair();  // This should be your actual wallet

// This is a mock function representing you generating a zk-proof with LightProtocol
async function generateProof() {
    // Replace this with the actual LightProtocol implementation
    return new Uint8Array([1, 2, 3, 4]);  // Mock proof
}

async function sendProofToSolana() {
    const proof = await generateProof();

    // The programId should be the address of your Solana program
    const programId = new solanaWeb3.PublicKey('YourProgramAddressHere');
    
    // Create transaction instruction
    const instruction = new solanaWeb3.TransactionInstruction({
        keys: [{
            pubkey: wallet.publicKey,
            isSigner: true,
            isWritable: true
        }],
        programId,
        data: proof,  // Your zk-proof data
    });

    // Create and send the transaction
    const transaction = new solanaWeb3.Transaction().add(instruction);
    transaction.feePayer = wallet.publicKey;
    const transactionId = await solanaWeb3.sendAndConfirmTransaction(
        connection,
        transaction,
        [wallet],
    );

    console.log('Proof sent with transaction ID', transactionId);
}

sendProofToSolana().catch(err => console.error(err));