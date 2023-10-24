// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use tw_coin_entry::error::SigningErrorType;
use tw_encoding::hex::ToHex;
use tw_memory::test_utils::tw_data_helper::TWDataHelper;
use tw_proto::EthereumRlp::Proto as RlpProto;
use tw_proto::{deserialize, serialize};
use wallet_core_rs::ffi::ethereum::rlp::tw_ethereum_rlp_encode;
use RlpProto::mod_RlpItem::OneOfitem as Item;

const ETHEREUM_COIN_TYPE: u32 = 60;

#[test]
fn test_ethereum_rlp() {
    let item = RlpProto::RlpItem {
        item: Item::number_u64(128),
    };
    let input = RlpProto::EncodingInput { item: Some(item) };
    let input_data = TWDataHelper::create(serialize(&input).unwrap());

    let output_data =
        TWDataHelper::wrap(unsafe { tw_ethereum_rlp_encode(ETHEREUM_COIN_TYPE, input_data.ptr()) })
            .to_vec()
            .expect("!tw_ethereum_rlp_encode returned nullptr");
    let output: RlpProto::EncodingOutput =
        deserialize(&output_data).expect("!tw_ethereum_rlp_encode returned an invalid output");

    assert_eq!(output.error, SigningErrorType::OK);
    assert!(output.error_message.is_empty());

    let expected_encoded = "8180";
    assert_eq!(output.encoded.to_hex(), expected_encoded);
}
