package examples;

import static org.assertj.core.api.Assertions.assertThat;

import com.partisiablockchain.BlockchainAddress;
import com.partisiablockchain.language.abicodegen.CommentSection;
import com.partisiablockchain.language.junit.ContractBytes;
import com.partisiablockchain.language.junit.ContractTest;
import com.partisiablockchain.language.junit.JunitContractTest;
import com.partisiablockchain.language.testenvironment.zk.node.task.PendingInputId;
import com.secata.stream.BitOutput;
import com.secata.stream.CompactBitArray;
import java.nio.file.Path;
import java.util.List;

/** Test suite for the ZkAverageSalary contract. */
public final class ZkCommentSectionTest extends JunitContractTest {

  private static final ContractBytes CONTRACT_BYTES =
      ContractBytes.fromPaths(
          Path.of("../target/wasm32-unknown-unknown/release/comment_section.zkwa"),
          Path.of("../target/wasm32-unknown-unknown/release/comment_section.abi"),
          Path.of("../target/wasm32-unknown-unknown/release/comment_section_contract_runner"));

  private BlockchainAddress account1;
  private BlockchainAddress account2;
  private BlockchainAddress account3;
  private BlockchainAddress account4;
//  private BlockchainAddress account5;
//  private BlockchainAddress account6;
  private BlockchainAddress concatMessage;

  /** Deploys the contract. */
  @ContractTest
  public void deployZkContract() {
    account1 = blockchain.newAccount(2);
    account2 = blockchain.newAccount(3);
    account3 = blockchain.newAccount(4);
    account4 = blockchain.newAccount(5);
//    account5 = blockchain.newAccount(6);
//    account6 = blockchain.newAccount(7);

    byte[] initialize = CommentSection.initialize();

    concatMessage = blockchain.deployZkContract(account1, CONTRACT_BYTES, initialize, 1_200_000);

    CommentSection.ContractState state =
        CommentSection.ContractState.deserialize(blockchain.getContractState(concatMessage));

    assertThat(state.administrator()).isEqualTo(account1);
  }

  /** Sends secret input to the contract. */
  @ContractTest(previous = "deployZkContract")
  public void sendSecretInput() {

    CommentSection.ContractState state =
        CommentSection.ContractState.deserialize(blockchain.getContractState(concatMessage));
    assertThat(state).isNotNull();

    blockchain.sendSecretInput(
        concatMessage, account1, createSecretIntInput(0b1101), new byte[] {0x40});
    blockchain.sendSecretInput(
        concatMessage, account2, createSecretIntInput(0b0111), new byte[] {0x40});
    blockchain.sendSecretInput(
        concatMessage, account3, createSecretIntInput(0b1001), new byte[] {0x40});
  }

  /** Sends secret input to the contract several times. */
  @ContractTest(previous = "deployZkContract")
  public void sendManyInputs() {
    zkNodes.stop();

    PendingInputId account1Input =
        blockchain.sendSecretInput(
            concatMessage, account1, createSecretIntInput(0b1000), new byte[] {0x40});
    zkNodes.confirmInput(account1Input);

    PendingInputId account2Input =
        blockchain.sendSecretInput(
            concatMessage, account2, createSecretIntInput(0b0011), new byte[] {0x40});
    zkNodes.confirmInput(account2Input);

    PendingInputId account3Input =
        blockchain.sendSecretInput(
            concatMessage, account3, createSecretIntInput(0b0100), new byte[] {0x40});
    zkNodes.confirmInput(account3Input);

    PendingInputId account4Input =
        blockchain.sendSecretInput(
            concatMessage, account4, createSecretIntInput(0b0010), new byte[] {0x40});
    //zkNodes.confirmInput(account4Input);

//    PendingInputId account5Input =
//        blockchain.sendSecretInput(
//            concatMessage, account5, createSecretIntInput(23300), new byte[] {0x40});
//    zkNodes.confirmInput(account5Input);
//
//    blockchain.sendSecretInput(
//        concatMessage, account6, createSecretIntInput(40150), new byte[] {0x40});
  }

  /** Starts the ZK computation. */
  @ContractTest(previous = "sendSecretInput")
  public void startComputation() {
    byte[] startCompute = CommentSection.computeConcatMessage();
    blockchain.sendAction(account1, concatMessage, startCompute);
    CommentSection.ContractState state =
        CommentSection.ContractState.deserialize(blockchain.getContractState(concatMessage));
    assertThat(state.concatMessageResult()).isEqualTo(0b10010000011100001101);
  }

  /** Starts and completes the ZK computation. */
  @ContractTest(previous = "sendManyInputs")
  public void startComputationWithAllInput() {
    zkNodes.stop();
    List<PendingInputId> pendingInputs = zkNodes.getPendingInputs(concatMessage);

    assertThat(pendingInputs.size()).isEqualTo(1);

    zkNodes.confirmInput(pendingInputs.get(0));

    byte[] startCompute = CommentSection.computeConcatMessage();
    blockchain.sendAction(account1, concatMessage, startCompute);
    zkNodes.finishTasks();
    CommentSection.ContractState state =
        CommentSection.ContractState.deserialize(blockchain.getContractState(concatMessage));

    assertThat(state.concatMessageResult()).isEqualTo(0b0010000001000000001100001000);
  }

  @ContractTest(previous = "sendManyInputs")
  void startComputationMissingOneInput() {
    zkNodes.stop();
    byte[] startCompute = CommentSection.computeConcatMessage();
    blockchain.sendAction(account1, concatMessage, startCompute);

    zkNodes.zkCompute(concatMessage);
    zkNodes.finishTasks();

    CommentSection.ContractState state =
        CommentSection.ContractState.deserialize(blockchain.getContractState(concatMessage));
    assertThat(state.concatMessageResult()).isEqualTo(0b01000000001100001000);
  }

  CompactBitArray createSecretIntInput(int secret) {
    return BitOutput.serializeBits(output -> output.writeSignedInt(secret, 32));
  }
}
