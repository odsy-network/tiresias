use crypto_bigint::modular::runtime_mod::{DynResidue, DynResidueParams};
use crypto_bigint::{Concat, Encoding, NonZero};
use crypto_bigint::{U1024, U2048, U4096};

fn encrypt(encryption_key: U2048, plaintext: U2048, randomness: U2048) -> U4096 {
    let N = encryption_key;
    let N2: U4096 = N.square();
    let N2_mod = DynResidueParams::new(&N2);

    let m = DynResidue::new(&U2048::ZERO.concat(&plaintext), N2_mod);
    let r = DynResidue::new(&U2048::ZERO.concat(&randomness), N2_mod);
    let N = DynResidue::new(&U2048::ZERO.concat(&N), N2_mod);
    let one = DynResidue::one(N2_mod);

    let mut ciphertext = ((m * N) + one); // $ (m*N + 1) $
    let N: U2048 = encryption_key;
    let N: U4096 = U2048::ZERO.concat(&N);
    ciphertext *= (r.pow(&N)); // $ * (r^N) $

    ciphertext.retrieve()
}

// TODO: now we are panicking if the decryption key is 0, I think it's better to return an option.
fn decrypt(encryption_key: &U2048, decryption_key: &U4096, ciphertext: &U4096) -> U2048 {
    let N = encryption_key;
    let N2: U4096 = N.square();

    let N = U2048::ZERO.concat(&N);
    let N2_mod = DynResidueParams::new(&N2);

    let c = DynResidue::new(&ciphertext, N2_mod);
    let d = decryption_key;

    let plaintext = c.pow(&d); // $ c^d mod N^2 = (1 + N)^{m*d mod N} mod N^2 = (1 + m*d*N) mod N^2 $
    let plaintext = (plaintext - DynResidue::one(N2_mod)).retrieve(); // $ c^d mod N^2 - 1 = m*d*N mod N^2 $
    let plaintext = plaintext / NonZero::new(N).unwrap(); // $ (c^d mod N^2 - 1) / N = m*d*N / N mod N^2 = m*d mod N $
    let plaintext = U2048::from_le_slice(&plaintext.to_le_bytes()[0..256]); // Trim zero-padding post-division and convert to U2048

    // Finally take mod N
    let N = encryption_key;
    let N_mod = DynResidueParams::new(&N);
    let plaintext = DynResidue::new(&plaintext, N_mod).retrieve();

    plaintext
}

// fn decryption_share(encryption_key: &U2048, decryption_key_share: &U2048, ciphertext: &U4096) -> U2048 {
//         return c^{d_i}
// }

// fn combine_decryption_shares(decryption_shares: Vec<U2048>) -> U2048 {
//   sums up all decryption shares, assumes linear (additive) sharing.
// }

// shamir secret sharing in Z_{N*phi(n)} where phi(n) = ((p-1) * (q-1))

// fn partial_decryption<T: Num + Pow<T, Output = T> + Clone>(

//     decryption_key: &T,
//     ciphertext: &T,
// ) -> T {
//     ciphertext.clone().pow(decryption_key.clone()) // $ c^d mod N^2= (1 + N)^{m*d mod N} mod N^2 = (1 + m*d*N) mod N^2 $
// }
//
// fn decrypt_from_partial_decryptions<T: Num + Pow<T, Output = T> + Clone>(
//     partial_decryptions: HashMap<u8, T>,
//     lagrange_coeffecients: HashMap<u8, T>,
// ) -> T {
//     partial_decryptions
//         .iter()
//         .fold(T::one(), |acc, (i, partial_decryption)| {
//             acc * (partial_decryption
//                 .clone()
//                 .pow(lagrange_coeffecients.get(i).unwrap().clone())) // TODO: properly handle the case where lagrange_coeffecients isn't defined for i (no item in map)
//         })
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_bigint::CheckedSub;

    const N: U2048 = U2048::from_be_hex("97431848911c007fa3a15b718ae97da192e68a4928c0259f2d19ab58ed01f1aa930e6aeb81f0d4429ac2f037def9508b91b45875c11668cea5dc3d4941abd8fbb2d6c8750e88a69727f982e633051f60252ad96ba2e9c9204f4c766c1c97bc096bb526e4b7621ec18766738010375829657c77a23faf50e3a31cb471f72c7abecdec61bdf45b2c73c666aa3729add2d01d7d96172353380c10011e1db3c47199b72da6ae769690c883e9799563d6605e0670a911a57ab5efc69a8c5611f158f1ae6e0b1b6434bafc21238921dc0b98a294195e4e88c173c8dab6334b207636774daad6f35138b9802c1784f334a82cbff480bb78976b22bb0fb41e78fdcb8095");
    const N2: U4096 = U4096::from_be_hex("5960383b5378ad0607f0f270ce7fb6dcaba6506f9fc56deeffaf605c9128db8ccf063e2e8221a8bdf82c027741a0303b08eb71fa6225a03df18f24c473dc6d4d3d30eb9c52a233bbfe967d04011b95e8de5bc482c3c217bcfdeb4df6f57af6ba9c6d66c69fb03a70a41fe1e87975c85343ef7d572ca06a0139706b23ed2b73ad72cb1b7e2e41840115651897c8757b3da9af3a60eebb6396ffd193738b4f04aa6ece638cef1bf4e9c45cf57f8debeda8598cbef732484752f5380737ba75ee00bf1b146817b9ab336d0ce5540395377347c653d1c9d272127ff12b9a0721b8ef13ecd8a8379f1b9a358de2af2c4cd97564dbd5328c2fc13d56ee30c8a101d333f5406afb1f4417b49d7a629d5076726877df11f05c998ae365e374a0141f0b99802214532c97c1ebf9faf6e277a8f29dbd8f3eab72266e60a77784249694819e42877a5e826745c97f84a5f37002b74d83fc064cf094be0e706a6710d47d253c4532e6aa4a679a75fa1d860b39085dab03186c67248e6c92223682f58bd41b67143e299329ce3a8045f3a0124c3d0ef9f0f49374d89b37d9c3321feb2ab4117df4f68246724ce41cd765326457968d848afcc0735531e5de7fea88cf2eb35ac68710c6e79d5ad25df6c0393c0267f56e8eac90a52637abe3e606769e70b20560eaf70e0d531b11dca299104fa933f887d85fb5f72386c196e40f559baee356b9");
    const p: U1024 = U1024::from_be_hex("f1b165ca594631cd4dfe1f360eec12e934f8c0c36199e39e53e427b674160f062f8ca237cd0ff687b04c576d6803c3998ee799633521c56baaa74eaf5a44b9895974e221d2fb54b400a8fe0597ffec381b1ebadf5f851af0edf7f49725e899ddd1fd42864101b82878a97a9aa0c193b3b30750989953a7743597eee210452001");
    const q: U1024 = U1024::from_be_hex("a037500622d17a20eaf5aa15486d878cef4e76cb52d98b872f03e834804352e315404a2916aa5bb7b62c4650d97c8b163e9f74f72fbf49e54322280c17fc7aefa9eee8328586d91e957e2e79965009e9298337058e3771129a052bcd23eac78217e982d1b1768d06ef424ea68a417856d309499954721a1c64049dec298fe095");
    const plaintext: U2048 = U2048::from_be_hex("23f6379f4b0435dd50c0eb12454495c99db09aed97fe498c0dba7c51f6c52ab7b8d8ba47896ee0c43d567a1b3611cb2d53ee74574acc9c4520106c0f6e5d0376817febb477bb729405387b6ae6e213b3b34c0eb0cbe5dff49452979ab7f0b514560b5c9b659732efd0d67a3d7b7512a5d97f1bde1c2263f741838a7c62d78133396715c9568c0524e20a3147cda4510ef2f32cefa6fb92caf3a26da63aba3693efce706303fe399b6c86664b1ccaa9fe6e1505d82c4dd9b0a60ea29ec88a91bf2656a3927ad39d561bfe4009f94398a9a7782383f063adeb922275efd950ef3739dee7854bbf93f939a947e3aec7344135e6b0623aff35e802311c10ede8b0d4");
    const randomness: U2048 = U2048::from_be_hex("4aba7692cfc2e1a30d46dc393c4d406837df82896da97268b377b8455ce9364d93ff7d0c051eed84f2335eeae95eaf5182055a9738f62d37d06cf4b24c663006513c823418d63db307a96a1ec6c4089df23a7cc69c4c64f914420955a3468d93087feedea153e05d94d184e823796dd326f8f6444405665b9a6af3a5fedf4d0e787792667e6e73e4631ea2cbcf7baa58fff7eb25eb739c31fadac1cd066d97bcd822af06a1e4df4a2ab76d252ddb960bbdc333fd38c912d27fa775e598d856a87ce770b1379dde2fbfce8d82f8692e7e1b33130d556c97b690d0b5f7a2f8652b79a8f07a35d3c4b9074be68daa04f13e7c54124d9dd4fe794a49375131d9c0b1");
    const ciphertext: U4096 = U4096::from_be_hex("0d1a2a781bf90133552b120beb2745bbe02b47cc4e5cc65b6eb5294770bd44b52ce581c4aec199687283360ab0c46bb3f0bb33733dbbf2d7e95a7c600ed20e990e8c3133f7ec238c0b47882363df7748757717443a3d1f9e85f0fb27e665844f591a0f922f42436688a72a71bdf7e93c764a84aff5b813c034787f5cf35a7102fe3be8c670ac26b83b08dabca47d9156ce09d7349ac73d269b7355d5266720654b83b09857add1a6c0be4677115f461ea15907e1472d3d7dcde351f9eff7e43968ae7012a67eeca940c25d3dd5694c5bbf1ed702bfd2094e424bb17bbf00270ded29320cd2e50af2283121ecf5f8593de49b18e465f3b1e1a39daca4d7382e4a610bdbd21dfd343108085b6e2c743f295df3785d3766b56c36efc0ea10ba3de8c16c43fcc051e7c27d835a481c0fdd48819ca9398043689027b00b275ca048018788a5133b280981afb0d6da7e64f3cf5f9e39e501fe7b80807b872ece22f6e4b6b0d8279656ceef614c87ce7ee314a339ef44c3adc4f5e5451b2649c215a358c0682095e19d52ed454d5f4e364397928996823cb02c61f8304561cb21e3bd0f4399f283b0b1ded686ace5dc653b240760c6437323fab45418b904d2eef8ab0639b4cba7cccee58f471413505ca0f8bb5a859769ad9465ddac949d22114cacaeadb72962816c49f50adc6338da7a54bdda29f8e6e667d832bd9c9f9841be8b18");
    const d: U4096 = U4096::from_be_hex( "19d698592b9ccb2890fb84be46cd2b18c360153b740aeccb606cf4168ee2de399f05273182bf468978508a5f4869cb867b340e144838dfaf4ca9bfd38cd55dc2837688aed2dbd76d95091640c47b2037d3d0ca854ffb4c84970b86f905cef24e876ddc8ab9e04f2a5f171b9c7146776c469f0d90908aa436b710cf4489afc73cd3ee38bb81e80a22d5d9228b843f435c48c5eb40088623a14a12b44e2721b56625da5d56d257bb27662c6975630d51e8f5b930d05fc5ba461a0e158cbda0f3266408c9bf60ff617e39ae49e707cbb40958adc512f3b4b69a5c3dc8b6d34cf45bc9597840057438598623fb65254869a165a6030ec6bec12fd59e192b3c1eefd33ef5d9336e0666aa8f36c6bd2749f86ea82290488ee31bf7498c2c77a8900bae00efcff418b62d41eb93502a245236b89c241ad6272724858122a2ebe1ae7ec4684b29048ba25b3a516c281a93043d58844cf3fa0c6f1f73db5db7ecba179652349dea8df5454e0205e910e0206736051ac4b7c707c3013e190423532e907af2e85e5bb6f6f0b9b58257ca1ec8b0318dd197f30352a96472a5307333f0e6b83f4f775fb302c1e10f21e1fcbfff17e3a4aa8bb6f553d9c6ebc2c884ae9b140dd66f21afc8610418e9f0ba2d14ecfa51ff08744a3470ebe4bb21bd6d65b58ac154630b8331ea620673ffbabb179a971a6577c407a076654a629c7733836c250000");

    #[test]
    fn test_encryption() {
        assert_eq!(encrypt(N, plaintext, randomness), ciphertext);
    }

    #[test]
    fn decrypts() {
        assert_eq!(plaintext, decrypt(&N, &d, &ciphertext));
    }
}
