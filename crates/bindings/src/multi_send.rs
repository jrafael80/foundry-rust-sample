pub use multi_send::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod multi_send {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("sendMany"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendMany"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address payable[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendManyERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendManyERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MultiSendERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MultiSendERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("total"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MultiSendETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("MultiSendETH"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("total"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MULTISEND_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW_\x80\xFD[Pa\t/\x80a\0\x1C_9_\xF3\xFE`\x80`@R`\x046\x10a\0(W_5`\xE0\x1C\x80c\x80\x86$y\x14a\0,W\x80c\xFC\x13?]\x14a\0AW[_\x80\xFD[a\0?a\0:6`\x04a\x06\xBAV[a\0`V[\0[4\x80\x15a\0LW_\x80\xFD[Pa\0?a\0[6`\x04a\x07\x7FV[a\x02SV[\x80Q\x82Q3\x91\x14a\0\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1A[\x9D\x98[\x1AY\x08\x1A[\x9C\x1D]`\x9A\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\xFF\x83Q\x11\x15a\0\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x19^\x18\xD9YY\x08\x1BX^\x08\x18[\x1B\x1B\xDD\xD9Y`r\x1B`D\x82\x01R`d\x01a\0\x9FV[_\x80[\x84Q\x81`\xFF\x16\x10\x15a\x02\nW_\x85\x82`\xFF\x16\x81Q\x81\x10a\x01\x14Wa\x01\x14a\x08UV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85\x83`\xFF\x16\x81Q\x81\x10a\x01:Wa\x01:a\x08UV[` \x02` \x01\x01Q`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x01\x7FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01\x84V[``\x91P[PP\x90P\x80a\x01\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs#0\xB4\xB62\xB2\x10:7\x909\xB2\xB72\x10\"\xBA42\xB9`a\x1B`D\x82\x01R`d\x01a\0\x9FV[\x84\x82`\xFF\x16\x81Q\x81\x10a\x01\xE1Wa\x01\xE1a\x08UV[` \x02` \x01\x01Q\x83a\x01\xF4\x91\x90a\x08}V[\x92PP\x80\x80a\x02\x02\x90a\x08\x90V[\x91PPa\0\xF2V[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F\xBF\x91\rv\x7F\x89\xAA\xD95\0\xE3\x0B\x9D}g\x86e\xB6v:\x16er\nwVlBh\xB8\xDE\x1F\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPV[\x80Q\x82Q3\x91\x14a\x02\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1A[\x9D\x98[\x1AY\x08\x1A[\x9C\x1D]`\x9A\x1B`D\x82\x01R`d\x01a\0\x9FV[`\xFF\x83Q\x11\x15a\x02\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x19^\x18\xD9YY\x08\x1BX^\x08\x18[\x1B\x1B\xDD\xD9Y`r\x1B`D\x82\x01R`d\x01a\0\x9FV[\x83_\x80[\x85Q\x81`\xFF\x16\x10\x15a\x03\x81Wa\x03E\x84\x87\x83`\xFF\x16\x81Q\x81\x10a\x03\x06Wa\x03\x06a\x08UV[` \x02` \x01\x01Q\x87\x84`\xFF\x16\x81Q\x81\x10a\x03#Wa\x03#a\x08UV[` \x02` \x01\x01Q\x86`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD5\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x81`\xFF\x16\x81Q\x81\x10a\x03ZWa\x03Za\x08UV[` \x02` \x01\x01Q\x82a\x03m\x91\x90a\x08}V[\x91P\x80a\x03y\x81a\x08\x90V[\x91PPa\x02\xE1V[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x82R\x85\x16` \x82\x01R\x90\x81\x01\x82\x90R\x7F\xE0\xEBz\xA3\x19\xC1\x08\xB4\xAA\xE8$\xC2\x81J\xDD\\m'\xDD\xA4\x06\x1DNV\xFD\x07\xEB\x98\x92\xC9\xFB+\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x04/\x90\x85\x90a\x045V[PPPPV[_a\x04I`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x04\x9BV[\x90P\x80Q_\x14\x15\x80\x15a\x04mWP\x80\x80` \x01\x90Q\x81\x01\x90a\x04k\x91\x90a\x08\xAEV[\x15[\x15a\x04\x96W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\x9FV[PPPV[``a\x04\xA8\x83\x83_a\x04\xB1V[\x90P[\x92\x91PPV[``\x81G\x10\x15a\x04\xD6W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\0\x9FV[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x04\xF1\x91\x90a\x08\xCDV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x05+W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x050V[``\x91P[P\x91P\x91Pa\x05@\x86\x83\x83a\x05LV[\x92PPP[\x93\x92PPPV[``\x82a\x05aWa\x05\\\x82a\x05\xA8V[a\x05EV[\x81Q\x15\x80\x15a\x05xWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x05\xA1W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\x9FV[P\x80a\x05EV[\x80Q\x15a\x05\xB8W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x11Wa\x06\x11a\x05\xD4V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x062Wa\x062a\x05\xD4V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xD1W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x06_W_\x80\xFD[\x815a\x06ra\x06m\x82a\x06\x19V[a\x05\xE8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x06\x93W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x06\xB0W\x805\x83R` \x92\x83\x01\x92\x01a\x06\x98V[P\x95\x94PPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x06\xCBW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xE1W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x06\xF1W_\x80\xFD[\x805a\x06\xFFa\x06m\x82a\x06\x19V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\x07 W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x07KW\x835a\x07:\x81a\x06<V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\x07'V[\x94PPPP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07iW_\x80\xFD[a\x07u\x85\x82\x86\x01a\x06PV[\x91PP\x92P\x92\x90PV[_\x80_``\x84\x86\x03\x12\x15a\x07\x91W_\x80\xFD[\x835a\x07\x9C\x81a\x06<V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xB7W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x07\xC7W_\x80\xFD[\x805a\x07\xD5a\x06m\x82a\x06\x19V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a\x07\xF6W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x08!W\x835a\x08\x10\x81a\x06<V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\x07\xFDV[\x94PPPP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08?W_\x80\xFD[a\x08K\x86\x82\x87\x01a\x06PV[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\xABWa\x04\xABa\x08iV[_`\xFF\x82\x16`\xFF\x81\x03a\x08\xA5Wa\x08\xA5a\x08iV[`\x01\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x08\xBEW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05EW_\x80\xFD[_\x82Q_[\x81\x81\x10\x15a\x08\xECW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x08\xD2V[P_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \xFF\xB1\x89\x95\xCEu_\xE9)\x02\xF7C8+v|z$s~\xC5v\xA8\xD2\xE7\xA1\x92H\xBC\xCB\x80\xEBdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static MULTISEND_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0(W_5`\xE0\x1C\x80c\x80\x86$y\x14a\0,W\x80c\xFC\x13?]\x14a\0AW[_\x80\xFD[a\0?a\0:6`\x04a\x06\xBAV[a\0`V[\0[4\x80\x15a\0LW_\x80\xFD[Pa\0?a\0[6`\x04a\x07\x7FV[a\x02SV[\x80Q\x82Q3\x91\x14a\0\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1A[\x9D\x98[\x1AY\x08\x1A[\x9C\x1D]`\x9A\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\xFF\x83Q\x11\x15a\0\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x19^\x18\xD9YY\x08\x1BX^\x08\x18[\x1B\x1B\xDD\xD9Y`r\x1B`D\x82\x01R`d\x01a\0\x9FV[_\x80[\x84Q\x81`\xFF\x16\x10\x15a\x02\nW_\x85\x82`\xFF\x16\x81Q\x81\x10a\x01\x14Wa\x01\x14a\x08UV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85\x83`\xFF\x16\x81Q\x81\x10a\x01:Wa\x01:a\x08UV[` \x02` \x01\x01Q`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x01\x7FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x01\x84V[``\x91P[PP\x90P\x80a\x01\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs#0\xB4\xB62\xB2\x10:7\x909\xB2\xB72\x10\"\xBA42\xB9`a\x1B`D\x82\x01R`d\x01a\0\x9FV[\x84\x82`\xFF\x16\x81Q\x81\x10a\x01\xE1Wa\x01\xE1a\x08UV[` \x02` \x01\x01Q\x83a\x01\xF4\x91\x90a\x08}V[\x92PP\x80\x80a\x02\x02\x90a\x08\x90V[\x91PPa\0\xF2V[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x81\x01\x83\x90R\x7F\xBF\x91\rv\x7F\x89\xAA\xD95\0\xE3\x0B\x9D}g\x86e\xB6v:\x16er\nwVlBh\xB8\xDE\x1F\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPV[\x80Q\x82Q3\x91\x14a\x02\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1A[\x9D\x98[\x1AY\x08\x1A[\x9C\x1D]`\x9A\x1B`D\x82\x01R`d\x01a\0\x9FV[`\xFF\x83Q\x11\x15a\x02\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x19^\x18\xD9YY\x08\x1BX^\x08\x18[\x1B\x1B\xDD\xD9Y`r\x1B`D\x82\x01R`d\x01a\0\x9FV[\x83_\x80[\x85Q\x81`\xFF\x16\x10\x15a\x03\x81Wa\x03E\x84\x87\x83`\xFF\x16\x81Q\x81\x10a\x03\x06Wa\x03\x06a\x08UV[` \x02` \x01\x01Q\x87\x84`\xFF\x16\x81Q\x81\x10a\x03#Wa\x03#a\x08UV[` \x02` \x01\x01Q\x86`\x01`\x01`\xA0\x1B\x03\x16a\x03\xD5\x90\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x81`\xFF\x16\x81Q\x81\x10a\x03ZWa\x03Za\x08UV[` \x02` \x01\x01Q\x82a\x03m\x91\x90a\x08}V[\x91P\x80a\x03y\x81a\x08\x90V[\x91PPa\x02\xE1V[P`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x82R\x85\x16` \x82\x01R\x90\x81\x01\x82\x90R\x7F\xE0\xEBz\xA3\x19\xC1\x08\xB4\xAA\xE8$\xC2\x81J\xDD\\m'\xDD\xA4\x06\x1DNV\xFD\x07\xEB\x98\x92\xC9\xFB+\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x84\x16`D\x82\x01R`d\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x84\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c#\xB8r\xDD`\xE0\x1B\x17\x90Ra\x04/\x90\x85\x90a\x045V[PPPPV[_a\x04I`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x04\x9BV[\x90P\x80Q_\x14\x15\x80\x15a\x04mWP\x80\x80` \x01\x90Q\x81\x01\x90a\x04k\x91\x90a\x08\xAEV[\x15[\x15a\x04\x96W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\x9FV[PPPV[``a\x04\xA8\x83\x83_a\x04\xB1V[\x90P[\x92\x91PPV[``\x81G\x10\x15a\x04\xD6W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\0\x9FV[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x04\xF1\x91\x90a\x08\xCDV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x05+W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x050V[``\x91P[P\x91P\x91Pa\x05@\x86\x83\x83a\x05LV[\x92PPP[\x93\x92PPPV[``\x82a\x05aWa\x05\\\x82a\x05\xA8V[a\x05EV[\x81Q\x15\x80\x15a\x05xWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x05\xA1W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\x9FV[P\x80a\x05EV[\x80Q\x15a\x05\xB8W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x06\x11Wa\x06\x11a\x05\xD4V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x062Wa\x062a\x05\xD4V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xD1W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x06_W_\x80\xFD[\x815a\x06ra\x06m\x82a\x06\x19V[a\x05\xE8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x06\x93W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x06\xB0W\x805\x83R` \x92\x83\x01\x92\x01a\x06\x98V[P\x95\x94PPPPPV[_\x80`@\x83\x85\x03\x12\x15a\x06\xCBW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\xE1W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x06\xF1W_\x80\xFD[\x805a\x06\xFFa\x06m\x82a\x06\x19V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\x07 W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x07KW\x835a\x07:\x81a\x06<V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\x07'V[\x94PPPP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07iW_\x80\xFD[a\x07u\x85\x82\x86\x01a\x06PV[\x91PP\x92P\x92\x90PV[_\x80_``\x84\x86\x03\x12\x15a\x07\x91W_\x80\xFD[\x835a\x07\x9C\x81a\x06<V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xB7W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x07\xC7W_\x80\xFD[\x805a\x07\xD5a\x06m\x82a\x06\x19V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a\x07\xF6W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x08!W\x835a\x08\x10\x81a\x06<V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\x07\xFDV[\x94PPPP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08?W_\x80\xFD[a\x08K\x86\x82\x87\x01a\x06PV[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04\xABWa\x04\xABa\x08iV[_`\xFF\x82\x16`\xFF\x81\x03a\x08\xA5Wa\x08\xA5a\x08iV[`\x01\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x08\xBEW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05EW_\x80\xFD[_\x82Q_[\x81\x81\x10\x15a\x08\xECW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x08\xD2V[P_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \xFF\xB1\x89\x95\xCEu_\xE9)\x02\xF7C8+v|z$s~\xC5v\xA8\xD2\xE7\xA1\x92H\xBC\xCB\x80\xEBdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static MULTISEND_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MultiSend<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MultiSend<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MultiSend<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MultiSend<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MultiSend<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MultiSend)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MultiSend<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTISEND_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MULTISEND_ABI.clone(),
                MULTISEND_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `sendMany` (0x80862479) function
        pub fn send_many(
            &self,
            to: ::std::vec::Vec<::ethers::core::types::Address>,
            value: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 134, 36, 121], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendManyERC20` (0xfc133f5d) function
        pub fn send_many_erc20(
            &self,
            token_address: ::ethers::core::types::Address,
            to: ::std::vec::Vec<::ethers::core::types::Address>,
            value: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 19, 63, 93], (token_address, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MultiSendERC20` event
        pub fn multi_send_erc20_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MultiSendERC20Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MultiSendETH` event
        pub fn multi_send_eth_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MultiSendETHFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MultiSendEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MultiSend<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MultiSendErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MultiSendErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiSendErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MultiSendErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MultiSendErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for MultiSendErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for MultiSendErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for MultiSendErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for MultiSendErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for MultiSendErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MultiSendERC20", abi = "MultiSendERC20(address,address,uint256)")]
    pub struct MultiSendERC20Filter {
        pub token: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub total: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MultiSendETH", abi = "MultiSendETH(address,uint256)")]
    pub struct MultiSendETHFilter {
        pub from: ::ethers::core::types::Address,
        pub total: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MultiSendEvents {
        MultiSendERC20Filter(MultiSendERC20Filter),
        MultiSendETHFilter(MultiSendETHFilter),
    }
    impl ::ethers::contract::EthLogDecode for MultiSendEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MultiSendERC20Filter::decode_log(log) {
                return Ok(MultiSendEvents::MultiSendERC20Filter(decoded));
            }
            if let Ok(decoded) = MultiSendETHFilter::decode_log(log) {
                return Ok(MultiSendEvents::MultiSendETHFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MultiSendEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MultiSendERC20Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiSendETHFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MultiSendERC20Filter> for MultiSendEvents {
        fn from(value: MultiSendERC20Filter) -> Self {
            Self::MultiSendERC20Filter(value)
        }
    }
    impl ::core::convert::From<MultiSendETHFilter> for MultiSendEvents {
        fn from(value: MultiSendETHFilter) -> Self {
            Self::MultiSendETHFilter(value)
        }
    }
    ///Container type for all input parameters for the `sendMany` function with signature `sendMany(address[],uint256[])` and selector `0x80862479`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sendMany", abi = "sendMany(address[],uint256[])")]
    pub struct SendManyCall {
        pub to: ::std::vec::Vec<::ethers::core::types::Address>,
        pub value: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `sendManyERC20` function with signature `sendManyERC20(address,address[],uint256[])` and selector `0xfc133f5d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sendManyERC20",
        abi = "sendManyERC20(address,address[],uint256[])"
    )]
    pub struct SendManyERC20Call {
        pub token_address: ::ethers::core::types::Address,
        pub to: ::std::vec::Vec<::ethers::core::types::Address>,
        pub value: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MultiSendCalls {
        SendMany(SendManyCall),
        SendManyERC20(SendManyERC20Call),
    }
    impl ::ethers::core::abi::AbiDecode for MultiSendCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SendManyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendMany(decoded));
            }
            if let Ok(decoded) = <SendManyERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendManyERC20(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiSendCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SendMany(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendManyERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MultiSendCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SendMany(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendManyERC20(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SendManyCall> for MultiSendCalls {
        fn from(value: SendManyCall) -> Self {
            Self::SendMany(value)
        }
    }
    impl ::core::convert::From<SendManyERC20Call> for MultiSendCalls {
        fn from(value: SendManyERC20Call) -> Self {
            Self::SendManyERC20(value)
        }
    }
}
