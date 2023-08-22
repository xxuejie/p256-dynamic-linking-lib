use ckb_mock_tx_types::{MockCellDep, MockInfo, MockInput, MockTransaction, Resource};
use ckb_script::TransactionScriptsVerifier;
use ckb_types::{
    bytes::Bytes,
    core::{
        cell::{CellMetaBuilder, ResolvedTransaction},
        Capacity, DepType, ScriptHashType, TransactionBuilder,
    },
    packed::{CellDep, CellInput, CellOutput, OutPoint, Script},
    prelude::*,
};
use lazy_static::lazy_static;
use p256::ecdsa::{signature::Signer, SigningKey};
use proptest::prelude::*;
use rand::{rngs::StdRng, thread_rng, SeedableRng};
use std::sync::Arc;

lazy_static! {
    pub static ref P256_BIN: Bytes = Bytes::from(std::fs::read("../../p256-striped.so").expect("read"));
    pub static ref TEST_BIN: Bytes = Bytes::from(std::fs::read("../test").expect("read"));
}

fn random_out_point() -> OutPoint {
    let tx_hash = {
        let mut rng = thread_rng();
        let mut buf = [0u8; 32];
        rng.fill(&mut buf);
        buf.pack()
    };
    OutPoint::new(tx_hash, 0)
}

fn script_cell(script_data: &Bytes) -> MockCellDep {
    let out_point = random_out_point();
    let cell = CellOutput::new_builder()
        .capacity(
            Capacity::bytes(script_data.len())
                .expect("script capacity")
                .pack(),
        )
        .build();

    MockCellDep {
        cell_dep: CellDep::new_builder()
            .out_point(out_point)
            .dep_type(DepType::Code.into())
            .build(),
        output: cell,
        data: script_data.clone(),
        header: None,
    }
}

fn build_mock_transaction(witnesses: &[Bytes]) -> (MockTransaction, ResolvedTransaction) {
    let mut mock_info = MockInfo::default();
    let mut tx_builder = TransactionBuilder::default();

    let p256_cell_dep = script_cell(&P256_BIN);
    let test_cell_dep = script_cell(&TEST_BIN);

    tx_builder = tx_builder
        .cell_dep(p256_cell_dep.cell_dep.clone())
        .cell_dep(test_cell_dep.cell_dep.clone());
    mock_info.cell_deps.push(p256_cell_dep);
    mock_info.cell_deps.push(test_cell_dep);

    let mock_input = MockInput {
        input: CellInput::new_builder()
            .previous_output(random_out_point())
            .build(),
        output: CellOutput::new_builder()
            .capacity(Capacity::bytes(200).expect("capacity").pack())
            .lock(
                Script::new_builder()
                    .code_hash(CellOutput::calc_data_hash(&TEST_BIN))
                    .hash_type(ScriptHashType::Data1.into())
                    .build(),
            )
            .build(),
        data: Bytes::default(),
        header: None,
    };

    tx_builder = tx_builder.input(mock_input.input.clone());
    mock_info.inputs.push(mock_input);

    tx_builder = tx_builder.output(
        CellOutput::new_builder()
            .capacity(Capacity::bytes(199).expect("capacity").pack())
            .build(),
    );

    for witness in witnesses {
        tx_builder = tx_builder.witness(witness.pack());
    }

    let tx = tx_builder.build();

    let resolved_tx = ResolvedTransaction {
        transaction: tx.clone(),
        resolved_dep_groups: Vec::new(),
        resolved_inputs: mock_info
            .inputs
            .iter()
            .map(|input| {
                CellMetaBuilder::from_cell_output(input.output.clone(), input.data.clone()).build()
            })
            .collect(),
        resolved_cell_deps: mock_info
            .cell_deps
            .iter()
            .map(|cell_dep| {
                CellMetaBuilder::from_cell_output(cell_dep.output.clone(), cell_dep.data.clone())
                    .build()
            })
            .collect(),
    };

    let mock_tx = MockTransaction {
        tx: tx.data(),
        mock_info,
    };

    (mock_tx, resolved_tx)
}

proptest! {
    #[test]
    fn test_random_signature(seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);

        let signing_key = SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key();
        let public_key_bytes = public_key.to_sec1_bytes();

        let mut message = [0u8; 32];
        rng.fill_bytes(&mut message[..]);

        let (signature, _) = signing_key.try_sign(&message).expect("sign");

        let (mock_tx, resolved_tx) = build_mock_transaction(&[
            CellOutput::calc_data_hash(&P256_BIN).as_bytes(),
            Bytes::from(message.to_vec()),
            Bytes::from(public_key_bytes.to_vec()),
            Bytes::from(signature.to_vec()),
        ]);

        let resource = Resource::from_mock_tx(&mock_tx).expect("resource");

        // let repr_tx: ckb_mock_tx_types::ReprMockTransaction = mock_tx.into();
        // let json = serde_json::to_string_pretty(&repr_tx).expect("serde");
        // std::fs::write("dump.json", json).expect("write");

        let verifier = TransactionScriptsVerifier::new(Arc::new(resolved_tx), resource);
        // verifier.set_debug_printer(move |hash: &ckb_types::packed::Byte32, message: &str| {
        //     let prefix = format!("Script group: {:x}", hash);
        //     eprintln!("{} DEBUG OUTPUT: {}", prefix, message);
        // });

        verifier.verify(10_000_000).expect("success");
    }

    #[test]
    fn test_invalid_signature(seed: u64, flip_bit in 0..256usize) {
        let mut rng = StdRng::seed_from_u64(seed);

        let signing_key = SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key();
        let public_key_bytes = public_key.to_sec1_bytes();

        let mut message = [0u8; 32];
        rng.fill_bytes(&mut message[..]);

        let (signature, _) = signing_key.try_sign(&message).expect("sign");

        message[flip_bit / 8] ^= 1 << (flip_bit % 8);

        let (mock_tx, resolved_tx) = build_mock_transaction(&[
            CellOutput::calc_data_hash(&P256_BIN).as_bytes(),
            Bytes::from(message.to_vec()),
            Bytes::from(public_key_bytes.to_vec()),
            Bytes::from(signature.to_vec()),
        ]);

        let resource = Resource::from_mock_tx(&mock_tx).expect("resource");

        // let repr_tx: ckb_mock_tx_types::ReprMockTransaction = mock_tx.into();
        // let json = serde_json::to_string_pretty(&repr_tx).expect("serde");
        // std::fs::write("dump.json", json).expect("write");

        let verifier = TransactionScriptsVerifier::new(Arc::new(resolved_tx), resource);
        // verifier.set_debug_printer(move |hash: &ckb_types::packed::Byte32, message: &str| {
        //     let prefix = format!("Script group: {:x}", hash);
        //     eprintln!("{} DEBUG OUTPUT: {}", prefix, message);
        // });

        verifier.verify(10_000_000).expect_err("failure");
    }
}
