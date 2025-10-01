// Copyright (c) 2020-2025 Bitcoin Dev Kit Developers
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

use anyhow::anyhow;
use bdk_wallet::bitcoin::bip32::DerivationPath;
use bdk_wallet::bitcoin::secp256k1::Secp256k1;
use bdk_wallet::bitcoin::NetworkKind;
use bdk_wallet::descriptor;
use bdk_wallet::descriptor::IntoWalletDescriptor;
use bdk_wallet::keys::bip39::{Language, Mnemonic, WordCount};
use bdk_wallet::keys::{GeneratableKey, GeneratedKey};
use bdk_wallet::miniscript::Tap;
use std::str::FromStr;

/// This example demonstrates how to generate a mnemonic phrase
/// using BDK and use that mnemonic phrase to generate a descriptor string.
#[allow(clippy::print_stdout)]
fn main() -> Result<(), anyhow::Error> {
    let secp = Secp256k1::new();

    // In this example we are generating a 12 word mnemonic phrase
    // but it is also possible generate 15, 18, 21 and 24 words
    // using the respective `WordCount` variant.
    let mnemonic: GeneratedKey<_, Tap> =
        Mnemonic::generate((WordCount::Words12, Language::English))
            .map_err(|_| anyhow!("Mnemonic generation error"))?;

    println!("Mnemonic phrase: {}", *mnemonic);
    let mnemonic_with_passphrase = (mnemonic, None);

    // Define the external and internal derivation key path.
    let external_path = DerivationPath::from_str("m/86h/1h/0h/0").unwrap();
    let internal_path = DerivationPath::from_str("m/86h/1h/0h/1").unwrap();

    // Generate external and internal descriptors from the mnemonic phrase.
    let (external_descriptor, ext_keymap) =
        descriptor!(tr((mnemonic_with_passphrase.clone(), external_path)))?
            .into_wallet_descriptor(&secp, NetworkKind::Test)?;
    let (internal_descriptor, int_keymap) =
        descriptor!(tr((mnemonic_with_passphrase, internal_path)))?
            .into_wallet_descriptor(&secp, NetworkKind::Test)?;

    println!("tpub external descriptor: {external_descriptor}");
    println!("tpub internal descriptor: {internal_descriptor}");
    println!(
        "tprv external descriptor: {}",
        external_descriptor.to_string_with_secret(&ext_keymap)
    );
    println!(
        "tprv internal descriptor: {}",
        internal_descriptor.to_string_with_secret(&int_keymap)
    );

    Ok(())
}
