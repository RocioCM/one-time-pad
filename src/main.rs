fn main() {
    // Our encrypted strings under the same key.
    let texts = vec!["160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e",
    "050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740",
    "000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c",
    "0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0145124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f",
    "031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41",
    "0b4916060808001a542e0002101309050a45500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c",
    "011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c",
    "0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c",
    "1f3cb1f3e01f3fd1f3ea1f3e61f3e01f3e71f3b31f3a91f3c81f3a91f3f91f3fc1f3fb1f3ec1f3e51f3f01f3a91f3f91f3ec1f3ec526e1b014a020411074c17111b1c071c4e4f0146430d0d08131d1d010707040017091648461e1d0618444f074c010e19594f0f1f1a07024e1d041719164e1c1652114f411645541b004e244f080213010c004c3b4c0911040e480e070b00310213101c4d0d4e00360b4f151a005253184913040e115454084f010f114554111d1a550f0d520401461f3e01f3e71f3e81f3e71f3ea1f3e01f3e81f3e51f3a91f3e01f3e71f3fa1f3fd1f3e01f3fd1f3fc1f3fd1f3e01f3e61f3e71f3a7",
    "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",];

    let pattern = String::from("bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.");

    match hex::decode(texts[8]) {
        Ok(xa) => {
            crib_utf8(&pattern, &xa);
        }
        Err(_) => (),
    };

    return;
    for i in 0..texts.len() {
        for j in i + 1..texts.len() {
            // println!("\n --- {} {}", i, j);
            let xor_msg_bytes = xor_ciphers_bytes(texts[i], texts[j]);

            // println!("<<{:?}>>", xor_msg_bytes);

            // 0. the Times 03/Jan/2009 Chancellor on brink of second bailout for banks.
            // 1. governments are good at cutting off the heads of a centrally controlled networks like Napster, but pure P2P networks like Gnutella and Tor seem to be holding their own.
            // 2. bitcoin is great as a form of digital money, but its scripting language is too weak for any kind of serious advanced applications to be built on top.
            // 3. in order to have a decentralised database, you need to have security. In order to have security, you need to have incentives.
            // 4. as society becomes more and more complex, cheating will in many ways become progressively easier and easier to do and harder to police or even understand.
            // 5. began to realise new possibilities opening up between the fields of ICT and game theory, and the inevitable social change to which this would lead.
            // 6. cryptocurrencies allowed non-custodial exchange, without users having to sign up or create accounts.
            // 7. not your keys, Not your coins.
            // 8.
            // 9. bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.
            crib(&pattern, &xor_msg_bytes);
        }
    }
}

fn xor_ciphers_bytes(cipher1: &str, cipher2: &str) -> Vec<u8> {
    let mut xor_result = Vec::new();

    let hex_a = hex::decode(cipher1);
    let hex_b = hex::decode(cipher2);

    match hex_a {
        Ok(xa) => match hex_b {
            Ok(xb) => {
                let len = xa.len().min(xb.len());

                for i in 0..len {
                    let key_a = xa[i];
                    let key_b = xb[i];
                    let key_char = key_a ^ key_b;
                    xor_result.push(key_char);
                    // println!(
                    //     "msg xor msg: {:?}, {:?}, {}, {}",
                    //     key_a, key_b, key_char, key_char as char
                    // );
                }
            }
            Err(_) => (),
        },
        Err(_) => (),
    }

    xor_result
}

fn crib(pattern: &String, string: &Vec<u8>) {
    // 1. Turn pattern string into bytes array.
    let pattern_bytes = Vec::from(pattern.as_bytes());

    // 2. Match pattern with each substring slice of the string.
    let len = if string.len() > pattern.len() {
        string.len() - pattern.len() + 1
    } else {
        1
    };
    for i in 0..len {
        let max = string.len().min(i + pattern.len());
        let substring = &string[i..max];
        let mut xor_result = String::new();
        let mut hex_result = Vec::<u8>::new();

        // 3. XOR every char of the substring.
        for k in 0..substring.len() {
            let xa = pattern_bytes[k];
            let xb = substring[k];
            let key_char = xb ^ xa;
            // println!("{}, {}, {}, {}", xa, xb, key_char, key_char as char);
            xor_result.push(key_char as char);
            hex_result.push(key_char);
        }
        // 4. Print decoded substring.
        // match String::from_utf8(hex_result) {
        //     Ok(string) => println!("Str {}", string),
        //     Err(_) => (),
        // };

        println!("{} {} \n", i, xor_result);
    }
}

fn crib_utf8(pattern: &String, string: &Vec<u8>) {
    let pattern_bytes = Vec::from(pattern.as_bytes());

    // 2. Match pattern with each substring slice of the string.
    let len = if string.len() > pattern.len() {
        string.len() - pattern.len() + 1
    } else {
        1
    };
    for i in 0..len {
        let max = string.len().min(i + pattern.len());

        for i in 0..(string.len() / 4) {
            let slice = string[(i * 4)..(i * 4 + 4)].to_vec();
            println!("Slice: {:?}", String::from_utf8(slice).unwrap());
        }

        return;

        let substring = &string[i..max];
        let mut xor_result = String::new();
        let mut hex_result = Vec::<u8>::new();

        // 3. XOR every char of the substring.
        for k in 0..substring.len() {
            let xa = pattern_bytes[k];
            let xb = substring[k];
            let key_char = xb ^ xa;
            // println!("{}, {}, {}, {}", xa, xb, key_char, key_char as char);
            xor_result.push(key_char as char);
            hex_result.push(key_char);
        }
        // 4. Print decoded substring.
        // match String::from_utf8(hex_result) {
        //     Ok(string) => println!("Str {}", string),
        //     Err(_) => (),
        // };

        println!("{} {} \n", i, xor_result);
    }
}
