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
                    ::std::borrow::ToOwned::to_owned("executeOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeOperation"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                                    name: ::std::borrow::ToOwned::to_owned("premiums"),
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
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("flashLoanPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashLoanPool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x19\xEB8\x03\x80b\0\x19\xEB\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01EV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xE1\x91\x90b\0\x01wV[\x15b\0\x013W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FNot a valid Augustus address\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\x01\x9BV[`\0` \x82\x84\x03\x12\x15b\0\x01XW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01pW`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15b\0\x01\x8AW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x01pW`\0\x80\xFD[`\x80Qa\x18-b\0\x01\xBE`\09`\0\x81\x81a\x01\x01\x01Ra\x0C\n\x01Ra\x18-`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0fW\x80c\x8D\xA5\xCB[\x14a\x01KW\x80c\x92\x0F\\\x84\x14a\x01^W\x80c\xB1\t\x9D\xAF\x14a\0\xA3W\x80c\xF0O'\x07\x14a\x01\x81W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x94W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\xA3W\x80c6\xF2GW\x14a\0\xDBW\x80c:\x82\x98g\x14a\0\xFCW\x80cBL&[\x14a\x01#W\x80cW\x05\xAEC\x14a\x018W[`\0\x80\xFD[a\0\xBEsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEEa\0\xE96`\x04a\x0E4V[a\x01\xA7V[`@Q\x90\x81R` \x01a\0\xD2V[a\0\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x016a\x0116`\x04a\x0E\xB9V[a\x03\0V[\0[a\x016a\x01F6`\x04a\x0E\xDDV[a\x03\xB4V[`\0Ta\0\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01qa\x01l6`\x04a\x0FUV[a\x04\x8FV[`@Q\x90\x15\x15\x81R` \x01a\0\xD2V[a\x016a\x01\x8F6`\x04a\x11\xD6V[a\x06\x05V[a\x016a\x01\xA26`\x04a\x0E\xB9V[a\x06\xC0V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02F\x91\x90a\x13\x07V[\x90P`\0\x83\x89\x89\x88\x88`@Q` \x01a\x02c\x95\x94\x93\x92\x91\x90a\x13IV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02\x7F\x88\x88\x83a\x075V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE9\x91\x90a\x13\x07V[a\x02\xF3\x91\x90a\x14MV[\x99\x98PPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81RsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xB0\x91\x90a\x14tV[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x04\x1EW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04\x19W=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x19\x91\x90a\x14tV[`\x003syJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x14a\x05\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7Fonly the lending pool can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01a\x01\xD2V[`\x01`\x01`\xA0\x1B\x03\x84\x160\x14a\x05qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7Fthe ape did not initiate this fl`D\x82\x01Rf0\xB9\xB467\xB0\xB7`\xC9\x1B`d\x82\x01R`\x84\x01a\x01\xD2V[`\0\x86\x86`\0\x81\x81\x10a\x05\x86Wa\x05\x86a\x14\x96V[\x90P` \x02\x015\x89\x89`\0\x81\x81\x10a\x05\xA0Wa\x05\xA0a\x14\x96V[\x90P` \x02\x015a\x05\xB1\x91\x90a\x14\xACV[\x90Pa\x05\xF3\x81\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x92Pa\t\xA6\x91PPV[P`\x01\x9B\x9APPPPPPPPPPPV[3s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x14a\x06hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the vault\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD2V[`\0\x82`\0\x81Q\x81\x10a\x06}Wa\x06}a\x14\x96V[` \x02` \x01\x01Q\x84`\0\x81Q\x81\x10a\x06\x98Wa\x06\x98a\x14\x96V[` \x02` \x01\x01Qa\x06\xAA\x91\x90a\x14\xACV[\x90Pa\x06\xB8\x81\x83`\x01a\t\xA6V[PPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[a\x07?\x83\x83a\x0BYV[\x15a\x08LW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x07zWa\x07za\x14\x96V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x07\xCBWa\x07\xCBa\x14\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc.\x1C\"O`\xE1\x1B\x81Rs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x90c\\8D\x9E\x90a\x08\x13\x900\x90\x86\x90\x86\x90\x89\x90`\x04\x01a\x15KV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08AW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x08\x82Wa\x08\x82a\x14\x96V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x08\xD3Wa\x08\xD3a\x14\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\t\x17Wa\t\x17a\x14\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\xAB\x9CK]`\xE0\x1B\x81R0\x90`\0\x90syJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x90c\xAB\x9CK]\x90a\tj\x90\x85\x90\x89\x90\x89\x90\x89\x90\x84\x90\x8E\x90\x8A\x90`\x04\x01a\x15\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x98W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\t\xC3\x91\x90a\x16\x9BV[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R\x94\x99P\x92\x97P\x90\x95P\x93P\x91PsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n8W=`\0\x80>=`\0\xFD[PPPPa\nE\x85a\x0B\xE5V[P\x86\x15a\n\xC3W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBD\x91\x90a\x14tV[Pa\x0BJV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81RsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BH\x91\x90a\x14tV[P[P`\x01\x98\x97PPPPPPPPV[`@Qcp\xA0\x821`\xE0\x1B\x81Rs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8`\x04\x82\x01R`\0\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xDA\x91\x90a\x13\x07V[\x10\x15\x90P[\x92\x91PPV[\x80Q`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R`\0\x92\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CSW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cw\x91\x90a\x14tV[a\x0C\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoINVALID_AUGUSTUS`\x80\x1B`D\x82\x01R`d\x01a\x01\xD2V[`\0\x83` \x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r#\x91\x90a\x17\xBEV[``\x86\x01Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x91\x92P\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x9F\x91\x90a\x14tV[P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x80\x01Q`@Qa\r\xBE\x91\x90a\x17\xDBV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\xFBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\0V[``\x91P[PP\x90P\x80a\x0E\x13W=`\0\x80>=`\0\xFD[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E1W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0EMW`\0\x80\xFD[\x865a\x0EX\x81a\x0E\x1CV[\x95P` \x87\x015a\x0Eh\x81a\x0E\x1CV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x99W`\0\x80\xFD[\x87\x01`\xA0\x81\x8A\x03\x12\x15a\x0E\xABW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x0E\xCBW`\0\x80\xFD[\x815a\x0E\xD6\x81a\x0E\x1CV[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xF0W`\0\x80\xFD[\x825a\x0E\xFB\x81a\x0E\x1CV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0F\x1BW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0FNW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\x0FsW`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x8BW`\0\x80\xFD[a\x0F\x97\x8D\x83\x8E\x01a\x0F\tV[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x0F\xB0W`\0\x80\xFD[a\x0F\xBC\x8D\x83\x8E\x01a\x0F\tV[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x0F\xD5W`\0\x80\xFD[a\x0F\xE1\x8D\x83\x8E\x01a\x0F\tV[\x90\x97P\x95P``\x8C\x015\x91Pa\x0F\xF6\x82a\x0E\x1CV[\x90\x93P`\x80\x8B\x015\x90\x80\x82\x11\x15a\x10\x0CW`\0\x80\xFD[\x81\x8C\x01\x91P\x8C`\x1F\x83\x01\x12a\x10 W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10/W`\0\x80\xFD[\x8D` \x82\x85\x01\x01\x11\x15a\x10AW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x93Wa\x10\x93a\x10ZV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC2Wa\x10\xC2a\x10ZV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xE4Wa\x10\xE4a\x10ZV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x10\xFFW`\0\x80\xFD[\x815` a\x11\x14a\x11\x0F\x83a\x10\xCAV[a\x10\x99V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x116W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x11RW\x805\x83R\x91\x83\x01\x91\x83\x01a\x11;V[P\x96\x95PPPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11wWa\x11wa\x10ZV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x11\x96W`\0\x80\xFD[\x815a\x11\xA4a\x11\x0F\x82a\x11]V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11\xB9W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x11\xECW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\x04W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x12\x18W`\0\x80\xFD[\x815` a\x12(a\x11\x0F\x83a\x10\xCAV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8B\x84\x11\x15a\x12GW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x12nW\x855a\x12_\x81a\x0E\x1CV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x12LV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x12\x84W`\0\x80\xFD[a\x12\x90\x88\x83\x89\x01a\x10\xEEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x12\xA6W`\0\x80\xFD[a\x12\xB2\x88\x83\x89\x01a\x10\xEEV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x12\xC8W`\0\x80\xFD[Pa\x12\xD5\x87\x82\x88\x01a\x11\x85V[\x91PP\x92\x95\x91\x94P\x92PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x13\x19W`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\xA0\x81R`\0\x865a\x13Z\x81a\x0E\x1CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x84\x01R` \x88\x015\x90a\x13y\x82a\x0E\x1CV[\x90\x81\x16`\xC0\x84\x01R`@\x88\x015\x90a\x13\x90\x82a\x0E\x1CV[\x16`\xE0\x83\x01R``\x87\x015a\x01\0\x83\x01R`\x80\x87\x0156\x88\x90\x03`\x1E\x19\x01\x81\x12a\x13\xB9W`\0\x80\xFD[\x87\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xD6W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x13\xE5W`\0\x80\xFD[`\xA0a\x01 \x85\x01Ra\x13\xFCa\x01@\x85\x01\x82\x84a\x13 V[\x92PPPa\x14\x15` \x83\x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R``\x82\x01\x93\x90\x93R`\x80\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x14mWa\x14ma\x147V[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\x86W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E\xD6W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xDFWa\x0B\xDFa\x147V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x14\xF0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x14\xD4V[P\x94\x95\x94PPPPPV[`\0[\x83\x81\x10\x15a\x15\x16W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xFEV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x157\x81` \x86\x01` \x86\x01a\x14\xFBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R`\0\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a\x15\x99W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x15{V[PP\x85\x81\x03`@\x87\x01Ra\x15\xAD\x81\x89a\x14\xBFV[\x93PPPP\x82\x81\x03``\x84\x01Ra\x15\xC4\x81\x85a\x15\x1FV[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x16\x1EW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x16\0V[PP\x85\x81\x03`@\x87\x01Ra\x162\x81\x8Ca\x14\xBFV[\x93PPPP\x82\x81\x03``\x84\x01Ra\x16I\x81\x88a\x14\xBFV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x16l\x81\x86a\x15\x1FV[\x91PPa\x16\x7F`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[\x80Qa\x16\x96\x81a\x0E\x1CV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x16\xB3W`\0\x80\xFD[\x85Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xCBW`\0\x80\xFD[\x90\x87\x01\x90`\xA0\x82\x8A\x03\x12\x15a\x16\xDFW`\0\x80\xFD[a\x16\xE7a\x10pV[\x82Qa\x16\xF2\x81a\x0E\x1CV[\x81R` \x83\x81\x01Qa\x17\x03\x81a\x0E\x1CV[\x82\x82\x01R`@\x84\x01Qa\x17\x15\x81a\x0E\x1CV[`@\x83\x01R``\x84\x81\x01Q\x90\x83\x01R`\x80\x84\x01Q\x83\x81\x11\x15a\x176W`\0\x80\xFD[\x80\x85\x01\x94PP\x8A`\x1F\x85\x01\x12a\x17KW`\0\x80\xFD[\x83Q\x92Pa\x17[a\x11\x0F\x84a\x11]V[\x83\x81R\x8B\x82\x85\x87\x01\x01\x11\x15a\x17oW`\0\x80\xFD[a\x17~\x84\x83\x83\x01\x84\x88\x01a\x14\xFBV[\x80`\x80\x84\x01RP\x81\x98Pa\x17\x93\x81\x8B\x01a\x16\x8BV[\x97PPPPPa\x17\xA5`@\x87\x01a\x16\x8BV[``\x87\x01Q`\x80\x90\x97\x01Q\x95\x98\x94\x97P\x95\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17\xD0W`\0\x80\xFD[\x81Qa\x0E\xD6\x81a\x0E\x1CV[`\0\x82Qa\x17\xED\x81\x84` \x87\x01a\x14\xFBV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xA3\xA5/@\x83\x1C\x11KT\x1D\x15\x84~d\x02\x19\xB9;\x10\xCF%\x1B\xBBt\xA7\xF8J\xB7u\xFB\xDA-dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0fW\x80c\x8D\xA5\xCB[\x14a\x01KW\x80c\x92\x0F\\\x84\x14a\x01^W\x80c\xB1\t\x9D\xAF\x14a\0\xA3W\x80c\xF0O'\x07\x14a\x01\x81W\x80c\xF2\xFD\xE3\x8B\x14a\x01\x94W`\0\x80\xFD[\x80c\x16\xF0\x11[\x14a\0\xA3W\x80c6\xF2GW\x14a\0\xDBW\x80c:\x82\x98g\x14a\0\xFCW\x80cBL&[\x14a\x01#W\x80cW\x05\xAEC\x14a\x018W[`\0\x80\xFD[a\0\xBEsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xEEa\0\xE96`\x04a\x0E4V[a\x01\xA7V[`@Q\x90\x81R` \x01a\0\xD2V[a\0\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x016a\x0116`\x04a\x0E\xB9V[a\x03\0V[\0[a\x016a\x01F6`\x04a\x0E\xDDV[a\x03\xB4V[`\0Ta\0\xBE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01qa\x01l6`\x04a\x0FUV[a\x04\x8FV[`@Q\x90\x15\x15\x81R` \x01a\0\xD2V[a\x016a\x01\x8F6`\x04a\x11\xD6V[a\x06\x05V[a\x016a\x01\xA26`\x04a\x0E\xB9V[a\x06\xC0V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02F\x91\x90a\x13\x07V[\x90P`\0\x83\x89\x89\x88\x88`@Q` \x01a\x02c\x95\x94\x93\x92\x91\x90a\x13IV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x02\x7F\x88\x88\x83a\x075V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xE9\x91\x90a\x13\x07V[a\x02\xF3\x91\x90a\x14MV[\x99\x98PPPPPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81RsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD`\x04\x82\x01R`\0\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xB0\x91\x90a\x14tV[PPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x04\x1EW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90`\0\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04\x19W=`\0\x80>=`\0\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x19\x91\x90a\x14tV[`\x003syJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x14a\x05\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7Fonly the lending pool can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01a\x01\xD2V[`\x01`\x01`\xA0\x1B\x03\x84\x160\x14a\x05qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7Fthe ape did not initiate this fl`D\x82\x01Rf0\xB9\xB467\xB0\xB7`\xC9\x1B`d\x82\x01R`\x84\x01a\x01\xD2V[`\0\x86\x86`\0\x81\x81\x10a\x05\x86Wa\x05\x86a\x14\x96V[\x90P` \x02\x015\x89\x89`\0\x81\x81\x10a\x05\xA0Wa\x05\xA0a\x14\x96V[\x90P` \x02\x015a\x05\xB1\x91\x90a\x14\xACV[\x90Pa\x05\xF3\x81\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x92Pa\t\xA6\x91PPV[P`\x01\x9B\x9APPPPPPPPPPPV[3s\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x14a\x06hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7Fcaller is not the vault\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xD2V[`\0\x82`\0\x81Q\x81\x10a\x06}Wa\x06}a\x14\x96V[` \x02` \x01\x01Q\x84`\0\x81Q\x81\x10a\x06\x98Wa\x06\x98a\x14\x96V[` \x02` \x01\x01Qa\x06\xAA\x91\x90a\x14\xACV[\x90Pa\x06\xB8\x81\x83`\x01a\t\xA6V[PPPPPPV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x01\xD2\x90a\x12\xE1V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[a\x07?\x83\x83a\x0BYV[\x15a\x08LW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x07zWa\x07za\x14\x96V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x07\xCBWa\x07\xCBa\x14\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc.\x1C\"O`\xE1\x1B\x81Rs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8\x90c\\8D\x9E\x90a\x08\x13\x900\x90\x86\x90\x86\x90\x89\x90`\x04\x01a\x15KV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08-W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08AW=`\0\x80>=`\0\xFD[PPPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x08\x82Wa\x08\x82a\x14\x96V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x83\x81`\0\x81Q\x81\x10a\x08\xD3Wa\x08\xD3a\x14\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\t\x17Wa\t\x17a\x14\x96V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\xAB\x9CK]`\xE0\x1B\x81R0\x90`\0\x90syJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x90c\xAB\x9CK]\x90a\tj\x90\x85\x90\x89\x90\x89\x90\x89\x90\x84\x90\x8E\x90\x8A\x90`\x04\x01a\x15\xCFV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\t\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\t\x98W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\t\xC3\x91\x90a\x16\x9BV[`@Qc\xFD!\xEC\xFF`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R\x94\x99P\x92\x97P\x90\x95P\x93P\x91PsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD\x90c\xFD!\xEC\xFF\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n$W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n8W=`\0\x80>=`\0\xFD[PPPPa\nE\x85a\x0B\xE5V[P\x86\x15a\n\xC3W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\n\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBD\x91\x90a\x14tV[Pa\x0BJV[`@Qc\t^\xA7\xB3`\xE0\x1B\x81RsyJa5\x8DhEYO\x94\xDC\x1D\xB0*%+[H\x14\xAD`\x04\x82\x01R`$\x81\x01\x8A\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BH\x91\x90a\x14tV[P[P`\x01\x98\x97PPPPPPPPV[`@Qcp\xA0\x821`\xE0\x1B\x81Rs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8`\x04\x82\x01R`\0\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xDA\x91\x90a\x13\x07V[\x10\x15\x90P[\x92\x91PPV[\x80Q`@Qc\xFB\x04\xE1{`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R`\0\x92\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xFB\x04\xE1{\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CSW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cw\x91\x90a\x14tV[a\x0C\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RoINVALID_AUGUSTUS`\x80\x1B`D\x82\x01R`d\x01a\x01\xD2V[`\0\x83` \x01Q\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\xD2\xC4\xB5\x98`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r#\x91\x90a\x17\xBEV[``\x86\x01Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x91\x92P\x83\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x9F\x91\x90a\x14tV[P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16\x86`\x80\x01Q`@Qa\r\xBE\x91\x90a\x17\xDBV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r\xFBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E\0V[``\x91P[PP\x90P\x80a\x0E\x13W=`\0\x80>=`\0\xFD[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E1W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0EMW`\0\x80\xFD[\x865a\x0EX\x81a\x0E\x1CV[\x95P` \x87\x015a\x0Eh\x81a\x0E\x1CV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91P`\xA0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x99W`\0\x80\xFD[\x87\x01`\xA0\x81\x8A\x03\x12\x15a\x0E\xABW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x0E\xCBW`\0\x80\xFD[\x815a\x0E\xD6\x81a\x0E\x1CV[\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xF0W`\0\x80\xFD[\x825a\x0E\xFB\x81a\x0E\x1CV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0F\x1BW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x0FNW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a\x0FsW`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x8BW`\0\x80\xFD[a\x0F\x97\x8D\x83\x8E\x01a\x0F\tV[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a\x0F\xB0W`\0\x80\xFD[a\x0F\xBC\x8D\x83\x8E\x01a\x0F\tV[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a\x0F\xD5W`\0\x80\xFD[a\x0F\xE1\x8D\x83\x8E\x01a\x0F\tV[\x90\x97P\x95P``\x8C\x015\x91Pa\x0F\xF6\x82a\x0E\x1CV[\x90\x93P`\x80\x8B\x015\x90\x80\x82\x11\x15a\x10\x0CW`\0\x80\xFD[\x81\x8C\x01\x91P\x8C`\x1F\x83\x01\x12a\x10 W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x10/W`\0\x80\xFD[\x8D` \x82\x85\x01\x01\x11\x15a\x10AW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x93Wa\x10\x93a\x10ZV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xC2Wa\x10\xC2a\x10ZV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xE4Wa\x10\xE4a\x10ZV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x10\xFFW`\0\x80\xFD[\x815` a\x11\x14a\x11\x0F\x83a\x10\xCAV[a\x10\x99V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x116W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x11RW\x805\x83R\x91\x83\x01\x91\x83\x01a\x11;V[P\x96\x95PPPPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x11wWa\x11wa\x10ZV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x11\x96W`\0\x80\xFD[\x815a\x11\xA4a\x11\x0F\x82a\x11]V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11\xB9W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x11\xECW`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\x04W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x12\x18W`\0\x80\xFD[\x815` a\x12(a\x11\x0F\x83a\x10\xCAV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8B\x84\x11\x15a\x12GW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x12nW\x855a\x12_\x81a\x0E\x1CV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x12LV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x12\x84W`\0\x80\xFD[a\x12\x90\x88\x83\x89\x01a\x10\xEEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x12\xA6W`\0\x80\xFD[a\x12\xB2\x88\x83\x89\x01a\x10\xEEV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x12\xC8W`\0\x80\xFD[Pa\x12\xD5\x87\x82\x88\x01a\x11\x85V[\x91PP\x92\x95\x91\x94P\x92PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x13\x19W`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\xA0\x81R`\0\x865a\x13Z\x81a\x0E\x1CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xA0\x84\x01R` \x88\x015\x90a\x13y\x82a\x0E\x1CV[\x90\x81\x16`\xC0\x84\x01R`@\x88\x015\x90a\x13\x90\x82a\x0E\x1CV[\x16`\xE0\x83\x01R``\x87\x015a\x01\0\x83\x01R`\x80\x87\x0156\x88\x90\x03`\x1E\x19\x01\x81\x12a\x13\xB9W`\0\x80\xFD[\x87\x01` \x81\x01\x905g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\xD6W`\0\x80\xFD[\x806\x03\x82\x13\x15a\x13\xE5W`\0\x80\xFD[`\xA0a\x01 \x85\x01Ra\x13\xFCa\x01@\x85\x01\x82\x84a\x13 V[\x92PPPa\x14\x15` \x83\x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x85\x16`@\x83\x01R``\x82\x01\x93\x90\x93R`\x80\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x14mWa\x14ma\x147V[P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x14\x86W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0E\xD6W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\xDFWa\x0B\xDFa\x147V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x14\xF0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x14\xD4V[P\x94\x95\x94PPPPPV[`\0[\x83\x81\x10\x15a\x15\x16W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\xFEV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x157\x81` \x86\x01` \x86\x01a\x14\xFBV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R`\0\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a\x15\x99W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x15{V[PP\x85\x81\x03`@\x87\x01Ra\x15\xAD\x81\x89a\x14\xBFV[\x93PPPP\x82\x81\x03``\x84\x01Ra\x15\xC4\x81\x85a\x15\x1FV[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x16\x1EW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x16\0V[PP\x85\x81\x03`@\x87\x01Ra\x162\x81\x8Ca\x14\xBFV[\x93PPPP\x82\x81\x03``\x84\x01Ra\x16I\x81\x88a\x14\xBFV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x16l\x81\x86a\x15\x1FV[\x91PPa\x16\x7F`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[\x80Qa\x16\x96\x81a\x0E\x1CV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x16\xB3W`\0\x80\xFD[\x85Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xCBW`\0\x80\xFD[\x90\x87\x01\x90`\xA0\x82\x8A\x03\x12\x15a\x16\xDFW`\0\x80\xFD[a\x16\xE7a\x10pV[\x82Qa\x16\xF2\x81a\x0E\x1CV[\x81R` \x83\x81\x01Qa\x17\x03\x81a\x0E\x1CV[\x82\x82\x01R`@\x84\x01Qa\x17\x15\x81a\x0E\x1CV[`@\x83\x01R``\x84\x81\x01Q\x90\x83\x01R`\x80\x84\x01Q\x83\x81\x11\x15a\x176W`\0\x80\xFD[\x80\x85\x01\x94PP\x8A`\x1F\x85\x01\x12a\x17KW`\0\x80\xFD[\x83Q\x92Pa\x17[a\x11\x0F\x84a\x11]V[\x83\x81R\x8B\x82\x85\x87\x01\x01\x11\x15a\x17oW`\0\x80\xFD[a\x17~\x84\x83\x83\x01\x84\x88\x01a\x14\xFBV[\x80`\x80\x84\x01RP\x81\x98Pa\x17\x93\x81\x8B\x01a\x16\x8BV[\x97PPPPPa\x17\xA5`@\x87\x01a\x16\x8BV[``\x87\x01Q`\x80\x90\x97\x01Q\x95\x98\x94\x97P\x95\x94\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x17\xD0W`\0\x80\xFD[\x81Qa\x0E\xD6\x81a\x0E\x1CV[`\0\x82Qa\x17\xED\x81\x84` \x87\x01a\x14\xFBV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xA3\xA5/@\x83\x1C\x11KT\x1D\x15\x84~d\x02\x19\xB9;\x10\xCF%\x1B\xBBt\xA7\xF8J\xB7u\xFB\xDA-dsolcC\0\x08\x18\x003";
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
        ///Calls the contract's `executeOperation` (0x920f5c84) function
        pub fn execute_operation(
            &self,
            assets: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            premiums: ::std::vec::Vec<::ethers::core::types::U256>,
            initiator: ::ethers::core::types::Address,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [146, 15, 92, 132],
                    (assets, amounts, premiums, initiator, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flashLoanPool` (0xb1099daf) function
        pub fn flash_loan_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 9, 157, 175], ())
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
    ///Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `0x920f5c84`
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
        name = "executeOperation",
        abi = "executeOperation(address[],uint256[],uint256[],address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<::ethers::core::types::U256>,
        pub initiator: ::ethers::core::types::Address,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `flashLoanPool` function with signature `flashLoanPool()` and selector `0xb1099daf`
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
    #[ethcall(name = "flashLoanPool", abi = "flashLoanPool()")]
    pub struct FlashLoanPoolCall;
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
        ExecuteOperation(ExecuteOperationCall),
        FlashLoanPool(FlashLoanPoolCall),
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
            if let Ok(decoded) = <ExecuteOperationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteOperation(decoded));
            }
            if let Ok(decoded) = <FlashLoanPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FlashLoanPool(decoded));
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
                Self::ExecuteOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FlashLoanPool(element) => {
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
                Self::ExecuteOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoanPool(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<ExecuteOperationCall> for LiquidatorCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<FlashLoanPoolCall> for LiquidatorCalls {
        fn from(value: FlashLoanPoolCall) -> Self {
            Self::FlashLoanPool(value)
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
    ///Container type for all return fields from the `executeOperation` function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `0x920f5c84`
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
    pub struct ExecuteOperationReturn(pub bool);
    ///Container type for all return fields from the `flashLoanPool` function with signature `flashLoanPool()` and selector `0xb1099daf`
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
    pub struct FlashLoanPoolReturn(pub ::ethers::core::types::Address);
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
