pub use liquidator::*;
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
pub mod liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("augustusRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IParaSwapAugustusRegistry",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AUGUSTUS_REGISTRY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AUGUSTUS_REGISTRY"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IParaSwapAugustusRegistry",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approvePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationArg1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidationArg2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("psp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Liquidator.ParaSwapData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IL2Pool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("receiveFlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("receiveFlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeAmounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("userData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x16\x928\x03\x80b\0\x16\x92\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01EV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xE1\x91\x90b\0\x01wV[\x15b\0\x013W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FNot a valid Augustus address\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\x01\x9BV[`\0` \x82\x84\x03\x12\x15b\0\x01XW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01pW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0\x01\x8AW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x01pW`\0\x80\xFD[`\x80Qa\x14\xD5b\0\x01\xBD`\09`\0\x81\x81`\xEB\x01Ra\x07\xB0\x01Ra\x14\xD5`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80cW\x05\xAEC\x11a\0[W\x80cW\x05\xAEC\x14a\x01\"W\x80c\x8D\xA5\xCB[\x14a\x015W\x80c\xF0O'\x07\x14a\x01HW\x80c\xF2\xFD\xE3\x8B\x14a\x01[W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\x8DW\x80c6\xF2GW\x14a\0\xC5W\x80c:\x82\x98g\x14a\0\xE6W\x80cBL&[\x14a\x01\rW[`\0\x80\xFD[a\0\xA8syJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD8a\0\xD36`\x04a\x0C\xFBV[a\x01nV[`@Q\x90\x81R` \x01a\0\xBCV[a\0\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01 a\x01\x1B6`\x04a\r\x80V[a\x03\xC5V[\0[a\x01 a\x0106`\x04a\r\x9DV[a\x04yV[`\0Ta\0\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01 a\x01V6`\x04a\x0FEV[a\x05TV[a\x01 a\x01i6`\x04a\r\x80V[a\x07\x16V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\r\x91\x90a\x10vV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x02GWa\x02Ga\x10\x8FV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x02\x98Wa\x02\x98a\x10\x8FV[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8`\x01`\x01`\xA0\x1B\x03\x16c\\8D\x9E0\x84\x84\x89\x8F\x8F\x8E\x8E`@Q` \x01a\x02\xE3\x95\x94\x93\x92\x91\x90a\x10\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x11\x94\x93\x92\x91\x90a\x12\x0CV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03?W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x85\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xAD\x91\x90a\x10vV[a\x03\xB7\x91\x90a\x12\xBEV[\x9A\x99PPPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81RsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04u\x91\x90a\x12\xE5V[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x04\xE3W`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04\xDEW=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x050W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDE\x91\x90a\x12\xE5V[3s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x14a\x05\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the vault\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x99V[`\0\x80`\0\x80`\0\x85\x80` \x01\x90Q\x81\x01\x90a\x05\xD3\x91\x90a\x13\x17V[\x94P\x94P\x94P\x94P\x94P`\0\x87`\0\x81Q\x81\x10a\x05\xF2Wa\x05\xF2a\x10\x8FV[` \x02` \x01\x01Q\x89`\0\x81Q\x81\x10a\x06\rWa\x06\ra\x10\x8FV[` \x02` \x01\x01Qa\x06\x1F\x91\x90a\x14:V[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R\x90\x91PsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06vW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x8AW=`\0\x80>=`\0\xFD[PPPPa\x06\x97\x86a\x07\x8BV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\t\x91\x90a\x12\xE5V[PPPPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[\x80Q`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R`\0\x92\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1D\x91\x90a\x12\xE5V[a\x08\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoINVALID_AUGUSTUS`\x80\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0\x83` \x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC9\x91\x90a\x14SV[\x90Pa\x08\xE0`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\twV[``\x85\x01Qa\x08\xFB\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90a\twV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x80\x01Q`@Qa\t\x19\x91\x90a\x14pV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\tVW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t[V[``\x91P[PP\x90P\x80a\tnW=`\0\x80>=`\0\xFD[\x95\x94PPPPPV[\x80\x15\x80a\t\xF1WP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xEF\x91\x90a\x10vV[\x15[a\n\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x01\x99V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90R\x83Q\x80\x85\x01\x90\x94R\x80\x84R\x7FSafeERC20: low-level call failed\x90\x84\x01Ra\x04\xDE\x92\x86\x92\x91`\0\x91a\n\xEC\x91\x85\x16\x90\x84\x90a\x0BiV[\x80Q\x90\x91P\x15a\x04\xDEW\x80\x80` \x01\x90Q\x81\x01\x90a\x0B\n\x91\x90a\x12\xE5V[a\x04\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01\x99V[``a\x0Bx\x84\x84`\0\x85a\x0B\x82V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x0B\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\x99V[\x84;a\x0C1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\x99V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x0CM\x91\x90a\x14pV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0C\x8AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\x8FV[``\x91P[P\x91P\x91Pa\x0C\x9F\x82\x82\x86a\x0C\xAAV[\x97\x96PPPPPPPV[``\x83\x15a\x0C\xB9WP\x81a\x0B{V[\x82Q\x15a\x0C\xC9W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x91\x90a\x14\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xF8W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\r\x14W`\0\x80\xFD[\x865a\r\x1F\x81a\x0C\xE3V[\x95P` \x87\x015a\r/\x81a\x0C\xE3V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r`W`\0\x80\xFD[\x87\x01`\xA0\x81\x8A\x03\x12\x15a\rrW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\r\x92W`\0\x80\xFD[\x815a\x0B{\x81a\x0C\xE3V[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB0W`\0\x80\xFD[\x825a\r\xBB\x81a\x0C\xE3V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x02Wa\x0E\x02a\r\xC9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E1Wa\x0E1a\r\xC9V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0ESWa\x0ESa\r\xC9V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0EnW`\0\x80\xFD[\x815` a\x0E\x83a\x0E~\x83a\x0E9V[a\x0E\x08V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0E\xA5W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0E\xC1W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0E\xAAV[P\x96\x95PPPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xE6Wa\x0E\xE6a\r\xC9V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F\x05W`\0\x80\xFD[\x815a\x0F\x13a\x0E~\x82a\x0E\xCCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0F(W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0F[W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FsW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0F\x87W`\0\x80\xFD[\x815` a\x0F\x97a\x0E~\x83a\x0E9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8B\x84\x11\x15a\x0F\xB6W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0F\xDDW\x855a\x0F\xCE\x81a\x0C\xE3V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0F\xBBV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x0F\xF3W`\0\x80\xFD[a\x0F\xFF\x88\x83\x89\x01a\x0E]V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x10\x15W`\0\x80\xFD[a\x10!\x88\x83\x89\x01a\x0E]V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x107W`\0\x80\xFD[Pa\x10D\x87\x82\x88\x01a\x0E\xF4V[\x91PP\x92\x95\x91\x94P\x92PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x10\x88W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\xA0\x81R`\0\x865a\x10\xDF\x81a\x0C\xE3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x84\x01R` \x88\x015\x90a\x10\xFE\x82a\x0C\xE3V[\x90\x81\x16`\xC0\x84\x01R`@\x88\x015\x90a\x11\x15\x82a\x0C\xE3V[\x16`\xE0\x83\x01R``\x87\x015a\x01\0\x83\x01R`\x80\x87\x0156\x88\x90\x03`\x1E\x19\x01\x81\x12a\x11>W`\0\x80\xFD[\x87\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11[W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x11jW`\0\x80\xFD[`\xA0a\x01 \x85\x01Ra\x11\x81a\x01@\x85\x01\x82\x84a\x10\xA5V[\x92PPPa\x11\x9A` \x83\x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R``\x82\x01\x93\x90\x93R`\x80\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x11\xD7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xBFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x11\xF8\x81` \x86\x01` \x86\x01a\x11\xBCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R`\0\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a\x12ZW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x12<V[PP\x85\x81\x03`@\x87\x01R\x87Q\x80\x82R\x90\x82\x01\x93P\x91P\x80\x87\x01`\0[\x83\x81\x10\x15a\x12\x92W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a\x12vV[PPPP\x82\x81\x03``\x84\x01Ra\x0C\x9F\x81\x85a\x11\xE0V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x12\xDEWa\x12\xDEa\x12\xA8V[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\xF7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B{W`\0\x80\xFD[\x80Qa\x13\x12\x81a\x0C\xE3V[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x13/W`\0\x80\xFD[\x85Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13GW`\0\x80\xFD[\x90\x87\x01\x90`\xA0\x82\x8A\x03\x12\x15a\x13[W`\0\x80\xFD[a\x13ca\r\xDFV[\x82Qa\x13n\x81a\x0C\xE3V[\x81R` \x83\x81\x01Qa\x13\x7F\x81a\x0C\xE3V[\x82\x82\x01R`@\x84\x01Qa\x13\x91\x81a\x0C\xE3V[`@\x83\x01R``\x84\x81\x01Q\x90\x83\x01R`\x80\x84\x01Q\x83\x81\x11\x15a\x13\xB2W`\0\x80\xFD[\x80\x85\x01\x94PP\x8A`\x1F\x85\x01\x12a\x13\xC7W`\0\x80\xFD[\x83Q\x92Pa\x13\xD7a\x0E~\x84a\x0E\xCCV[\x83\x81R\x8B\x82\x85\x87\x01\x01\x11\x15a\x13\xEBW`\0\x80\xFD[a\x13\xFA\x84\x83\x83\x01\x84\x88\x01a\x11\xBCV[\x80`\x80\x84\x01RP\x81\x98Pa\x14\x0F\x81\x8B\x01a\x13\x07V[\x97PPPPPa\x14!`@\x87\x01a\x13\x07V[``\x87\x01Q`\x80\x90\x97\x01Q\x95\x98\x94\x97P\x95\x94\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x14MWa\x14Ma\x12\xA8V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14eW`\0\x80\xFD[\x81Qa\x0B{\x81a\x0C\xE3V[`\0\x82Qa\x14\x82\x81\x84` \x87\x01a\x11\xBCV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x0B{` \x83\x01\x84a\x11\xE0V\xFE\xA2dipfsX\"\x12 -\xFCB*`F\xC5+,o\xFC\x9A\\d\xBF\xA0h\xFEY}\xD4Uw\x96(d\xDAg\xC0\xE0\xA0\xE4dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x88W`\x005`\xE0\x1C\x80cW\x05\xAEC\x11a\0[W\x80cW\x05\xAEC\x14a\x01\"W\x80c\x8D\xA5\xCB[\x14a\x015W\x80c\xF0O'\x07\x14a\x01HW\x80c\xF2\xFD\xE3\x8B\x14a\x01[W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\x8DW\x80c6\xF2GW\x14a\0\xC5W\x80c:\x82\x98g\x14a\0\xE6W\x80cBL&[\x14a\x01\rW[`\0\x80\xFD[a\0\xA8syJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xD8a\0\xD36`\x04a\x0C\xFBV[a\x01nV[`@Q\x90\x81R` \x01a\0\xBCV[a\0\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01 a\x01\x1B6`\x04a\r\x80V[a\x03\xC5V[\0[a\x01 a\x0106`\x04a\r\x9DV[a\x04yV[`\0Ta\0\xA8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01 a\x01V6`\x04a\x0FEV[a\x05TV[a\x01 a\x01i6`\x04a\r\x80V[a\x07\x16V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\r\x91\x90a\x10vV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x02GWa\x02Ga\x10\x8FV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x02\x98Wa\x02\x98a\x10\x8FV[` \x02` \x01\x01\x81\x81RPPs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8`\x01`\x01`\xA0\x1B\x03\x16c\\8D\x9E0\x84\x84\x89\x8F\x8F\x8E\x8E`@Q` \x01a\x02\xE3\x95\x94\x93\x92\x91\x90a\x10\xCEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x11\x94\x93\x92\x91\x90a\x12\x0CV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03+W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03?W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x85\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xAD\x91\x90a\x10vV[a\x03\xB7\x91\x90a\x12\xBEV[\x9A\x99PPPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81RsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04u\x91\x90a\x12\xE5V[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x04\xE3W`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04\xDEW=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x050W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDE\x91\x90a\x12\xE5V[3s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x14a\x05\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the vault\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\x99V[`\0\x80`\0\x80`\0\x85\x80` \x01\x90Q\x81\x01\x90a\x05\xD3\x91\x90a\x13\x17V[\x94P\x94P\x94P\x94P\x94P`\0\x87`\0\x81Q\x81\x10a\x05\xF2Wa\x05\xF2a\x10\x8FV[` \x02` \x01\x01Q\x89`\0\x81Q\x81\x10a\x06\rWa\x06\ra\x10\x8FV[` \x02` \x01\x01Qa\x06\x1F\x91\x90a\x14:V[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x84\x90R\x90\x91PsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06vW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x8AW=`\0\x80>=`\0\xFD[PPPPa\x06\x97\x86a\x07\x8BV[P`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\t\x91\x90a\x12\xE5V[PPPPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x90a\x10PV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[\x80Q`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R`\0\x92\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1D\x91\x90a\x12\xE5V[a\x08\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoINVALID_AUGUSTUS`\x80\x1B`D\x82\x01R`d\x01a\x01\x99V[`\0\x83` \x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC9\x91\x90a\x14SV[\x90Pa\x08\xE0`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\twV[``\x85\x01Qa\x08\xFB\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x83\x90a\twV[`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x80\x01Q`@Qa\t\x19\x91\x90a\x14pV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\tVW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t[V[``\x91P[PP\x90P\x80a\tnW=`\0\x80>=`\0\xFD[\x95\x94PPPPPV[\x80\x15\x80a\t\xF1WP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xEF\x91\x90a\x10vV[\x15[a\n\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x01\x99V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`$\x83\x01R`D\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x83R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90R\x83Q\x80\x85\x01\x90\x94R\x80\x84R\x7FSafeERC20: low-level call failed\x90\x84\x01Ra\x04\xDE\x92\x86\x92\x91`\0\x91a\n\xEC\x91\x85\x16\x90\x84\x90a\x0BiV[\x80Q\x90\x91P\x15a\x04\xDEW\x80\x80` \x01\x90Q\x81\x01\x90a\x0B\n\x91\x90a\x12\xE5V[a\x04\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x01\x99V[``a\x0Bx\x84\x84`\0\x85a\x0B\x82V[\x90P[\x93\x92PPPV[``\x82G\x10\x15a\x0B\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x01\x99V[\x84;a\x0C1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x01\x99V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa\x0CM\x91\x90a\x14pV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0C\x8AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\x8FV[``\x91P[P\x91P\x91Pa\x0C\x9F\x82\x82\x86a\x0C\xAAV[\x97\x96PPPPPPPV[``\x83\x15a\x0C\xB9WP\x81a\x0B{V[\x82Q\x15a\x0C\xC9W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\x99\x91\x90a\x14\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xF8W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\r\x14W`\0\x80\xFD[\x865a\r\x1F\x81a\x0C\xE3V[\x95P` \x87\x015a\r/\x81a\x0C\xE3V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r`W`\0\x80\xFD[\x87\x01`\xA0\x81\x8A\x03\x12\x15a\rrW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\r\x92W`\0\x80\xFD[\x815a\x0B{\x81a\x0C\xE3V[`\0\x80`@\x83\x85\x03\x12\x15a\r\xB0W`\0\x80\xFD[\x825a\r\xBB\x81a\x0C\xE3V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x02Wa\x0E\x02a\r\xC9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E1Wa\x0E1a\r\xC9V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0ESWa\x0ESa\r\xC9V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0EnW`\0\x80\xFD[\x815` a\x0E\x83a\x0E~\x83a\x0E9V[a\x0E\x08V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0E\xA5W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0E\xC1W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0E\xAAV[P\x96\x95PPPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\xE6Wa\x0E\xE6a\r\xC9V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F\x05W`\0\x80\xFD[\x815a\x0F\x13a\x0E~\x82a\x0E\xCCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0F(W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0F[W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FsW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0F\x87W`\0\x80\xFD[\x815` a\x0F\x97a\x0E~\x83a\x0E9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8B\x84\x11\x15a\x0F\xB6W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0F\xDDW\x855a\x0F\xCE\x81a\x0C\xE3V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0F\xBBV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x0F\xF3W`\0\x80\xFD[a\x0F\xFF\x88\x83\x89\x01a\x0E]V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x10\x15W`\0\x80\xFD[a\x10!\x88\x83\x89\x01a\x0E]V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x107W`\0\x80\xFD[Pa\x10D\x87\x82\x88\x01a\x0E\xF4V[\x91PP\x92\x95\x91\x94P\x92PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x10\x88W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\xA0\x81R`\0\x865a\x10\xDF\x81a\x0C\xE3V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x84\x01R` \x88\x015\x90a\x10\xFE\x82a\x0C\xE3V[\x90\x81\x16`\xC0\x84\x01R`@\x88\x015\x90a\x11\x15\x82a\x0C\xE3V[\x16`\xE0\x83\x01R``\x87\x015a\x01\0\x83\x01R`\x80\x87\x0156\x88\x90\x03`\x1E\x19\x01\x81\x12a\x11>W`\0\x80\xFD[\x87\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11[W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x11jW`\0\x80\xFD[`\xA0a\x01 \x85\x01Ra\x11\x81a\x01@\x85\x01\x82\x84a\x10\xA5V[\x92PPPa\x11\x9A` \x83\x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R``\x82\x01\x93\x90\x93R`\x80\x01R\x93\x92PPPV[`\0[\x83\x81\x10\x15a\x11\xD7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x11\xBFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x11\xF8\x81` \x86\x01` \x86\x01a\x11\xBCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R`\0\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a\x12ZW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x12<V[PP\x85\x81\x03`@\x87\x01R\x87Q\x80\x82R\x90\x82\x01\x93P\x91P\x80\x87\x01`\0[\x83\x81\x10\x15a\x12\x92W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a\x12vV[PPPP\x82\x81\x03``\x84\x01Ra\x0C\x9F\x81\x85a\x11\xE0V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x12\xDEWa\x12\xDEa\x12\xA8V[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\xF7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0B{W`\0\x80\xFD[\x80Qa\x13\x12\x81a\x0C\xE3V[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x13/W`\0\x80\xFD[\x85Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13GW`\0\x80\xFD[\x90\x87\x01\x90`\xA0\x82\x8A\x03\x12\x15a\x13[W`\0\x80\xFD[a\x13ca\r\xDFV[\x82Qa\x13n\x81a\x0C\xE3V[\x81R` \x83\x81\x01Qa\x13\x7F\x81a\x0C\xE3V[\x82\x82\x01R`@\x84\x01Qa\x13\x91\x81a\x0C\xE3V[`@\x83\x01R``\x84\x81\x01Q\x90\x83\x01R`\x80\x84\x01Q\x83\x81\x11\x15a\x13\xB2W`\0\x80\xFD[\x80\x85\x01\x94PP\x8A`\x1F\x85\x01\x12a\x13\xC7W`\0\x80\xFD[\x83Q\x92Pa\x13\xD7a\x0E~\x84a\x0E\xCCV[\x83\x81R\x8B\x82\x85\x87\x01\x01\x11\x15a\x13\xEBW`\0\x80\xFD[a\x13\xFA\x84\x83\x83\x01\x84\x88\x01a\x11\xBCV[\x80`\x80\x84\x01RP\x81\x98Pa\x14\x0F\x81\x8B\x01a\x13\x07V[\x97PPPPPa\x14!`@\x87\x01a\x13\x07V[``\x87\x01Q`\x80\x90\x97\x01Q\x95\x98\x94\x97P\x95\x94\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x14MWa\x14Ma\x12\xA8V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14eW`\0\x80\xFD[\x81Qa\x0B{\x81a\x0C\xE3V[`\0\x82Qa\x14\x82\x81\x84` \x87\x01a\x11\xBCV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0a\x0B{` \x83\x01\x84a\x11\xE0V\xFE\xA2dipfsX\"\x12 -\xFCB*`F\xC5+,o\xFC\x9A\\d\xBF\xA0h\xFEY}\xD4Uw\x96(d\xDAg\xC0\xE0\xA0\xE4dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
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
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `AUGUSTUS_REGISTRY` (0x3a829867) function
        pub fn augustus_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([58, 130, 152, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvePool` (0x424c265b) function
        pub fn approve_pool(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 76, 38, 91], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x36f24757) function
        pub fn liquidate(
            &self,
            collateral: ::ethers::core::types::Address,
            debt: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            liquidation_arg_1: [u8; 32],
            liquidation_arg_2: [u8; 32],
            psp: ParaSwapData,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [54, 242, 71, 87],
                    (
                        collateral,
                        debt,
                        debt_to_cover,
                        liquidation_arg_1,
                        liquidation_arg_2,
                        psp,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pool` (0x16f0115b) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 240, 17, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `receiveFlashLoan` (0xf04f2707) function
        pub fn receive_flash_loan(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 79, 39, 7], (tokens, amounts, fee_amounts, user_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recover` (0x5705ae43) function
        pub fn recover(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 5, 174, 67], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `AUGUSTUS_REGISTRY` function with signature `AUGUSTUS_REGISTRY()` and selector `0x3a829867`
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
    #[ethcall(name = "AUGUSTUS_REGISTRY", abi = "AUGUSTUS_REGISTRY()")]
    pub struct AugustusRegistryCall;
    ///Container type for all input parameters for the `approvePool` function with signature `approvePool(address)` and selector `0x424c265b`
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
    #[ethcall(name = "approvePool", abi = "approvePool(address)")]
    pub struct ApprovePoolCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,address,uint256,bytes32,bytes32,(address,address,address,uint256,bytes))` and selector `0x36f24757`
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
        name = "liquidate",
        abi = "liquidate(address,address,uint256,bytes32,bytes32,(address,address,address,uint256,bytes))"
    )]
    pub struct LiquidateCall {
        pub collateral: ::ethers::core::types::Address,
        pub debt: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub liquidation_arg_1: [u8; 32],
        pub liquidation_arg_2: [u8; 32],
        pub psp: ParaSwapData,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pool` function with signature `pool()` and selector `0x16f0115b`
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
    #[ethcall(name = "pool", abi = "pool()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `receiveFlashLoan` function with signature `receiveFlashLoan(address[],uint256[],uint256[],bytes)` and selector `0xf04f2707`
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
        name = "receiveFlashLoan",
        abi = "receiveFlashLoan(address[],uint256[],uint256[],bytes)"
    )]
    pub struct ReceiveFlashLoanCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `recover` function with signature `recover(address,uint256)` and selector `0x5705ae43`
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
    #[ethcall(name = "recover", abi = "recover(address,uint256)")]
    pub struct RecoverCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum LiquidatorCalls {
        AugustusRegistry(AugustusRegistryCall),
        ApprovePool(ApprovePoolCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        Pool(PoolCall),
        ReceiveFlashLoan(ReceiveFlashLoanCall),
        Recover(RecoverCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AugustusRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AugustusRegistry(decoded));
            }
            if let Ok(decoded) = <ApprovePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApprovePool(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) = <ReceiveFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReceiveFlashLoan(decoded));
            }
            if let Ok(decoded) = <RecoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Recover(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AugustusRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApprovePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReceiveFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Recover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AugustusRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceiveFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AugustusRegistryCall> for LiquidatorCalls {
        fn from(value: AugustusRegistryCall) -> Self {
            Self::AugustusRegistry(value)
        }
    }
    impl ::core::convert::From<ApprovePoolCall> for LiquidatorCalls {
        fn from(value: ApprovePoolCall) -> Self {
            Self::ApprovePool(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ReceiveFlashLoanCall> for LiquidatorCalls {
        fn from(value: ReceiveFlashLoanCall) -> Self {
            Self::ReceiveFlashLoan(value)
        }
    }
    impl ::core::convert::From<RecoverCall> for LiquidatorCalls {
        fn from(value: RecoverCall) -> Self {
            Self::Recover(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `AUGUSTUS_REGISTRY` function with signature `AUGUSTUS_REGISTRY()` and selector `0x3a829867`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AugustusRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(address,address,uint256,bytes32,bytes32,(address,address,address,uint256,bytes))` and selector `0x36f24757`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidateReturn {
        pub collateral_gain: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pool` function with signature `pool()` and selector `0x16f0115b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolReturn(pub ::ethers::core::types::Address);
    ///`ParaSwapData(address,address,address,uint256,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParaSwapData {
        pub augustus: ::ethers::core::types::Address,
        pub src_asset: ::ethers::core::types::Address,
        pub dest_asset: ::ethers::core::types::Address,
        pub src_amount: ::ethers::core::types::U256,
        pub swap_call_data: ::ethers::core::types::Bytes,
    }
}
