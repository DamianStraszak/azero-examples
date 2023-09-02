#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod rec {
    pub trait RecursiveCall: ::ink::env::ContractEnv {
        /// Holds general and global information about the trait.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        type __ink_TraitInfo: ::ink::codegen::TraitCallForwarder;
        /// Output type of the respective trait message.
        type getOutput: ::ink::codegen::ImpliesReturn<u64>;
        fn get(&self) -> Self::getOutput;
        /// Output type of the respective trait message.
        type incrementOutput: ::ink::codegen::ImpliesReturn<()>;
        fn increment(&mut self) -> Self::incrementOutput;
        /// Output type of the respective trait message.
        type incrementRecursiveOutput: ::ink::codegen::ImpliesReturn<()>;
        fn increment_recursive(&mut self) -> Self::incrementRecursiveOutput;
    }
    const _: () = {
        impl<E> RecursiveCall for ::ink::reflect::TraitDefinitionRegistry<E>
        where
            E: ::ink::env::Environment,
        {
            /// Holds general and global information about the trait.
            #[allow(non_camel_case_types)]
            type __ink_TraitInfo = __ink_TraitInfoRecursiveCall<E>;
            type getOutput = u64;
            #[cold]
            fn get(&self) -> Self::getOutput {
                ::ink::codegen::utils::consume_type::<
                    ::ink::codegen::DispatchOutput<u64>,
                >();
                /// We enforce linking errors in case this is ever actually called.
                /// These linker errors are properly resolved by the cargo-contract tool.
                extern {
                    fn __ink_enforce_error_0x013452656375727369766543616c6c0c676574a30eae9d00() -> !;
                }
                unsafe {
                    __ink_enforce_error_0x013452656375727369766543616c6c0c676574a30eae9d00()
                }
            }
            type incrementOutput = ();
            #[cold]
            fn increment(&mut self) -> Self::incrementOutput {
                /// We enforce linking errors in case this is ever actually called.
                /// These linker errors are properly resolved by the cargo-contract tool.
                extern {
                    fn __ink_enforce_error_0x013452656375727369766543616c6c24696e6372656d656e74a74c110201() -> !;
                }
                unsafe {
                    __ink_enforce_error_0x013452656375727369766543616c6c24696e6372656d656e74a74c110201()
                }
            }
            type incrementRecursiveOutput = ();
            #[cold]
            fn increment_recursive(&mut self) -> Self::incrementRecursiveOutput {
                /// We enforce linking errors in case this is ever actually called.
                /// These linker errors are properly resolved by the cargo-contract tool.
                extern {
                    fn __ink_enforce_error_0x013452656375727369766543616c6c4c696e6372656d656e745f7265637572736976656c1ca23a01() -> !;
                }
                unsafe {
                    __ink_enforce_error_0x013452656375727369766543616c6c4c696e6372656d656e745f7265637572736976656c1ca23a01()
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct __ink_TraitInfoRecursiveCall<E> {
            marker: ::core::marker::PhantomData<fn() -> E>,
        }
        impl<E> ::ink::reflect::TraitMessageInfo<797334489u32>
        for __ink_TraitInfoRecursiveCall<E> {
            const PAYABLE: ::core::primitive::bool = false;
            const SELECTOR: [::core::primitive::u8; 4usize] = [
                0xA3_u8,
                0x0E_u8,
                0xAE_u8,
                0x9D_u8,
            ];
        }
        impl<E> ::ink::reflect::TraitMessageInfo<314397139u32>
        for __ink_TraitInfoRecursiveCall<E> {
            const PAYABLE: ::core::primitive::bool = false;
            const SELECTOR: [::core::primitive::u8; 4usize] = [
                0xA7_u8,
                0x4C_u8,
                0x11_u8,
                0x02_u8,
            ];
        }
        impl<E> ::ink::reflect::TraitMessageInfo<2141125528u32>
        for __ink_TraitInfoRecursiveCall<E> {
            const PAYABLE: ::core::primitive::bool = false;
            const SELECTOR: [::core::primitive::u8; 4usize] = [
                0x6C_u8,
                0x1C_u8,
                0xA2_u8,
                0x3A_u8,
            ];
        }
        impl<E> ::ink::reflect::TraitInfo for __ink_TraitInfoRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            const ID: u32 = 1750998437;
            const PATH: &'static ::core::primitive::str = "recursive::rec";
            const NAME: &'static ::core::primitive::str = "RecursiveCall";
        }
        impl<E> ::ink::codegen::TraitCallForwarder for __ink_TraitInfoRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            type Forwarder = __ink_TraitCallForwarderRecursiveCall<E>;
        }
        /// The global call builder type for all trait implementers.
        ///
        /// All calls to types (contracts) implementing the trait will be built by this type.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            account_id: <E as ::ink::env::Environment>::AccountId,
        }
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E> ::scale::Encode for __ink_TraitCallBuilderRecursiveCall<E>
            where
                E: ::ink::env::Environment,
                <E as ::ink::env::Environment>::AccountId: ::scale::Encode,
                <E as ::ink::env::Environment>::AccountId: ::scale::Encode,
            {
                fn size_hint(&self) -> usize {
                    ::scale::Encode::size_hint(&&self.account_id)
                }
                fn encode_to<
                    __CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::scale::Encode::encode_to(&&self.account_id, __codec_dest_edqy)
                }
                fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                    ::scale::Encode::encode(&&self.account_id)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::scale::Encode::using_encoded(&&self.account_id, f)
                }
            }
            #[automatically_derived]
            impl<E> ::scale::EncodeLike for __ink_TraitCallBuilderRecursiveCall<E>
            where
                E: ::ink::env::Environment,
                <E as ::ink::env::Environment>::AccountId: ::scale::Encode,
                <E as ::ink::env::Environment>::AccountId: ::scale::Encode,
            {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E> ::scale::Decode for __ink_TraitCallBuilderRecursiveCall<E>
            where
                E: ::ink::env::Environment,
                <E as ::ink::env::Environment>::AccountId: ::scale::Decode,
                <E as ::ink::env::Environment>::AccountId: ::scale::Decode,
            {
                fn decode<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    ::core::result::Result::Ok(__ink_TraitCallBuilderRecursiveCall::<E> {
                        account_id: {
                            let __codec_res_edqy = <<E as ::ink::env::Environment>::AccountId as ::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e
                                            .chain(
                                                "Could not decode `__ink_TraitCallBuilderRecursiveCall::account_id`",
                                            ),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::scale::DecodeFinished, ::scale::Error> {
                    match (
                        &::core::mem::size_of::<
                            <E as ::ink::env::Environment>::AccountId,
                        >(),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<
                        <E as ::ink::env::Environment>::AccountId,
                    >() > 0
                    {
                        1
                    } else {
                        0
                    } <= 1)
                    {
                        ::core::panicking::panic(
                            "assertion failed: if ::core::mem::size_of::<<E as ::ink::env::Environment>::AccountId>() > 0 {\\n            1\\n        } else { 0 } <= 1",
                        )
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<
                            <E as ::ink::env::Environment>::AccountId,
                        > = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<
                                    ::core::mem::MaybeUninit<
                                        <E as ::ink::env::Environment>::AccountId,
                                    >,
                                >()
                        };
                        <<E as ::ink::env::Environment>::AccountId as ::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[cfg(feature = "std")]
        impl<E> ::ink::storage::traits::StorageLayout
        for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::ink::storage::traits::StorageLayout,
        {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "__ink_TraitCallBuilderRecursiveCall",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "account_id",
                                <<E as ::ink::env::Environment>::AccountId as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                        ],
                    ),
                )
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E> ::core::clone::Clone for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::core::clone::Clone,
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    account_id: ::core::clone::Clone::clone(&self.account_id),
                }
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E> ::core::fmt::Debug for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::core::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct("__ink_TraitCallBuilderRecursiveCall")
                    .field("account_id", &self.account_id)
                    .finish()
            }
        }
        #[cfg(feature = "std")]
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E> ::scale_info::TypeInfo for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::scale_info::TypeInfo + 'static,
        {
            type Identity = <E as ::ink::env::Environment>::AccountId;
            fn type_info() -> ::scale_info::Type {
                <<E as ::ink::env::Environment>::AccountId as ::scale_info::TypeInfo>::type_info()
            }
        }
        impl<E> ::ink::env::call::FromAccountId<E>
        for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn from_account_id(
                account_id: <E as ::ink::env::Environment>::AccountId,
            ) -> Self {
                Self { account_id }
            }
        }
        impl<E, AccountId> ::core::convert::From<AccountId>
        for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment<AccountId = AccountId>,
            AccountId: ::ink::env::AccountIdGuard,
        {
            fn from(value: AccountId) -> Self {
                <Self as ::ink::env::call::FromAccountId<E>>::from_account_id(value)
            }
        }
        impl<E> ::ink::ToAccountId<E> for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn to_account_id(&self) -> <E as ::ink::env::Environment>::AccountId {
                <<E as ::ink::env::Environment>::AccountId as ::core::clone::Clone>::clone(
                    &self.account_id,
                )
            }
        }
        impl<E, AccountId> ::core::convert::AsRef<AccountId>
        for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment<AccountId = AccountId>,
        {
            fn as_ref(&self) -> &AccountId {
                &self.account_id
            }
        }
        impl<E, AccountId> ::core::convert::AsMut<AccountId>
        for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment<AccountId = AccountId>,
        {
            fn as_mut(&mut self) -> &mut AccountId {
                &mut self.account_id
            }
        }
        impl<E> ::ink::env::ContractEnv for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            type Env = E;
        }
        impl<E> RecursiveCall for __ink_TraitCallBuilderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            #[allow(non_camel_case_types)]
            type __ink_TraitInfo = __ink_TraitInfoRecursiveCall<E>;
            #[allow(clippy::type_complexity)]
            type getOutput = ::ink::env::call::CallBuilder<
                Self::Env,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Self::Env>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<u64>>,
            >;
            #[inline]
            fn get(&self) -> Self::getOutput {
                ::ink::env::call::build_call::<Self::Env>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0xA3_u8,
                                0x0E_u8,
                                0xAE_u8,
                                0x9D_u8,
                            ]),
                        ),
                    )
                    .returns::<u64>()
            }
            #[allow(clippy::type_complexity)]
            type incrementOutput = ::ink::env::call::CallBuilder<
                Self::Env,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Self::Env>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            >;
            #[inline]
            fn increment(&mut self) -> Self::incrementOutput {
                ::ink::env::call::build_call::<Self::Env>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0xA7_u8,
                                0x4C_u8,
                                0x11_u8,
                                0x02_u8,
                            ]),
                        ),
                    )
                    .returns::<()>()
            }
            #[allow(clippy::type_complexity)]
            type incrementRecursiveOutput = ::ink::env::call::CallBuilder<
                Self::Env,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Self::Env>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            >;
            #[inline]
            fn increment_recursive(&mut self) -> Self::incrementRecursiveOutput {
                ::ink::env::call::build_call::<Self::Env>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0x6C_u8,
                                0x1C_u8,
                                0xA2_u8,
                                0x3A_u8,
                            ]),
                        ),
                    )
                    .returns::<()>()
            }
        }
        /// The global call forwarder for the ink! trait definition.
        ///
        /// All cross-contract calls to contracts implementing the associated ink! trait
        /// will be handled by this type.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            builder: <Self as ::ink::codegen::TraitCallBuilder>::Builder,
        }
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E> ::scale::Encode for __ink_TraitCallForwarderRecursiveCall<E>
            where
                E: ::ink::env::Environment,
            {
                fn size_hint(&self) -> usize {
                    ::scale::Encode::size_hint(&&self.builder)
                }
                fn encode_to<
                    __CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::scale::Encode::encode_to(&&self.builder, __codec_dest_edqy)
                }
                fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                    ::scale::Encode::encode(&&self.builder)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::scale::Encode::using_encoded(&&self.builder, f)
                }
            }
            #[automatically_derived]
            impl<E> ::scale::EncodeLike for __ink_TraitCallForwarderRecursiveCall<E>
            where
                E: ::ink::env::Environment,
            {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E> ::scale::Decode for __ink_TraitCallForwarderRecursiveCall<E>
            where
                E: ::ink::env::Environment,
            {
                fn decode<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    ::core::result::Result::Ok(__ink_TraitCallForwarderRecursiveCall::<
                        E,
                    > {
                        builder: {
                            let __codec_res_edqy = <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e
                                            .chain(
                                                "Could not decode `__ink_TraitCallForwarderRecursiveCall::builder`",
                                            ),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::scale::DecodeFinished, ::scale::Error> {
                    match (
                        &::core::mem::size_of::<
                            <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                        >(),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<
                        <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                    >() > 0
                    {
                        1
                    } else {
                        0
                    } <= 1)
                    {
                        ::core::panicking::panic(
                            "assertion failed: if ::core::mem::size_of::<<Self as\\n                        ::ink::codegen::TraitCallBuilder>::Builder>() > 0 {\\n            1\\n        } else { 0 } <= 1",
                        )
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<
                            <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                        > = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<
                                    ::core::mem::MaybeUninit<
                                        <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                                    >,
                                >()
                        };
                        <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[cfg(feature = "std")]
        impl<E> ::ink::storage::traits::StorageLayout
        for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::ink::storage::traits::StorageLayout,
        {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::storage::traits::StorageLayout>::layout(
                    __key,
                )
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E> ::core::clone::Clone for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::core::clone::Clone,
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    builder: <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::core::clone::Clone>::clone(
                        &self.builder,
                    ),
                }
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E> ::core::fmt::Debug for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::core::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct("__ink_TraitCallForwarderRecursiveCall")
                    .field("account_id", &self.builder.account_id)
                    .finish()
            }
        }
        #[cfg(feature = "std")]
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E> ::scale_info::TypeInfo for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
            <E as ::ink::env::Environment>::AccountId: ::scale_info::TypeInfo + 'static,
        {
            type Identity = <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::scale_info::TypeInfo>::Identity;
            fn type_info() -> ::scale_info::Type {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::scale_info::TypeInfo>::type_info()
            }
        }
        impl<E> ::ink::env::call::FromAccountId<E>
        for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn from_account_id(
                account_id: <E as ::ink::env::Environment>::AccountId,
            ) -> Self {
                Self {
                    builder: <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::env::call::FromAccountId<
                        E,
                    >>::from_account_id(account_id),
                }
            }
        }
        impl<E, AccountId> ::core::convert::From<AccountId>
        for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment<AccountId = AccountId>,
            AccountId: ::ink::env::AccountIdGuard,
        {
            fn from(value: AccountId) -> Self {
                <Self as ::ink::env::call::FromAccountId<E>>::from_account_id(value)
            }
        }
        impl<E> ::ink::ToAccountId<E> for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn to_account_id(&self) -> <E as ::ink::env::Environment>::AccountId {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::ToAccountId<
                    E,
                >>::to_account_id(&self.builder)
            }
        }
        impl<E, AccountId> ::core::convert::AsRef<AccountId>
        for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment<AccountId = AccountId>,
        {
            fn as_ref(&self) -> &AccountId {
                <_ as ::core::convert::AsRef<AccountId>>::as_ref(&self.builder)
            }
        }
        impl<E, AccountId> ::core::convert::AsMut<AccountId>
        for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment<AccountId = AccountId>,
        {
            fn as_mut(&mut self) -> &mut AccountId {
                <_ as ::core::convert::AsMut<AccountId>>::as_mut(&mut self.builder)
            }
        }
        /// This trait allows to bridge from call forwarder to call builder.
        ///
        /// Also this explains why we designed the generated code so that we have
        /// both types and why the forwarder is a thin-wrapper around the builder
        /// as this allows to perform this operation safely.
        impl<E> ::ink::codegen::TraitCallBuilder
        for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            type Builder = __ink_TraitCallBuilderRecursiveCall<E>;
            #[inline]
            fn call(&self) -> &<Self as ::ink::codegen::TraitCallBuilder>::Builder {
                &self.builder
            }
            #[inline]
            fn call_mut(
                &mut self,
            ) -> &mut <Self as ::ink::codegen::TraitCallBuilder>::Builder {
                &mut self.builder
            }
        }
        impl<E> ::ink::env::ContractEnv for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            type Env = E;
        }
        impl<E> RecursiveCall for __ink_TraitCallForwarderRecursiveCall<E>
        where
            E: ::ink::env::Environment,
        {
            #[allow(non_camel_case_types)]
            type __ink_TraitInfo = __ink_TraitInfoRecursiveCall<E>;
            type getOutput = u64;
            #[inline]
            fn get(&self) -> Self::getOutput {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as RecursiveCall>::get(
                        <Self as ::ink::codegen::TraitCallBuilder>::call(self),
                    )
                    .try_invoke()
                    .unwrap_or_else(|env_err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0}: {1:?}",
                                "encountered error while calling <__ink_TraitCallForwarderRecursiveCall as RecursiveCall>::get",
                                env_err
                            ),
                        );
                    })
                    .unwrap_or_else(|lang_err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0}: {1:?}",
                                "encountered error while calling <__ink_TraitCallForwarderRecursiveCall as RecursiveCall>::get",
                                lang_err
                            ),
                        );
                    })
            }
            type incrementOutput = ();
            #[inline]
            fn increment(&mut self) -> Self::incrementOutput {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as RecursiveCall>::increment(
                        <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self),
                    )
                    .try_invoke()
                    .unwrap_or_else(|env_err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0}: {1:?}",
                                "encountered error while calling <__ink_TraitCallForwarderRecursiveCall as RecursiveCall>::increment",
                                env_err
                            ),
                        );
                    })
                    .unwrap_or_else(|lang_err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0}: {1:?}",
                                "encountered error while calling <__ink_TraitCallForwarderRecursiveCall as RecursiveCall>::increment",
                                lang_err
                            ),
                        );
                    })
            }
            type incrementRecursiveOutput = ();
            #[inline]
            fn increment_recursive(&mut self) -> Self::incrementRecursiveOutput {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as RecursiveCall>::increment_recursive(
                        <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self),
                    )
                    .try_invoke()
                    .unwrap_or_else(|env_err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0}: {1:?}",
                                "encountered error while calling <__ink_TraitCallForwarderRecursiveCall as RecursiveCall>::increment_recursive",
                                env_err
                            ),
                        );
                    })
                    .unwrap_or_else(|lang_err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "{0}: {1:?}",
                                "encountered error while calling <__ink_TraitCallForwarderRecursiveCall as RecursiveCall>::increment_recursive",
                                lang_err
                            ),
                        );
                    })
            }
        }
    };
}
mod recursive {
    impl ::ink::env::ContractEnv for Recursive {
        type Env = ::ink::env::DefaultEnvironment;
    }
    type Environment = <Recursive as ::ink::env::ContractEnv>::Env;
    type AccountId = <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
    type Balance = <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Balance;
    type Hash = <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Hash;
    type Timestamp = <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
    type BlockNumber = <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
    type ChainExtension = <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::ChainExtension;
    const MAX_EVENT_TOPICS: usize = <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::MAX_EVENT_TOPICS;
    const _: () = {
        struct Check {
            salt: (),
            field_0: u64,
        }
    };
    #[cfg(not(feature = "__ink_dylint_Storage"))]
    pub struct Recursive {
        value: <u64 as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<3355897524u32, ()>,
        >>::Type,
    }
    const _: () = {
        impl<
            __ink_generic_salt: ::ink::storage::traits::StorageKey,
        > ::ink::storage::traits::StorableHint<__ink_generic_salt> for Recursive {
            type Type = Recursive;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for Recursive {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for Recursive {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Recursive {
                    value: <<u64 as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<3355897524u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(__input)?,
                })
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __dest: &mut __ink_O,
            ) {
                match self {
                    Recursive { value: __binding_0 } => {
                        ::ink::storage::traits::Storable::encode(__binding_0, __dest);
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Recursive {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Recursive", "recursive::recursive"))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <u64 as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<3355897524u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("value")
                                    .type_name(
                                        "<u64 as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<3355897524u32, ()>,>>::Type",
                                    )
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for Recursive {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "Recursive",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "value",
                                <<u64 as ::ink::storage::traits::AutoStorableHint<
                                    ::ink::storage::traits::ManualKey<3355897524u32, ()>,
                                >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                        ],
                    ),
                )
            }
        }
    };
    const _: () = {
        impl ::ink::reflect::ContractName for Recursive {
            const NAME: &'static str = "Recursive";
        }
    };
    const _: () = {
        impl<'a> ::ink::codegen::Env for &'a Recursive {
            type EnvAccess = ::ink::EnvAccess<
                'a,
                <Recursive as ::ink::env::ContractEnv>::Env,
            >;
            fn env(self) -> Self::EnvAccess {
                <<Self as ::ink::codegen::Env>::EnvAccess as ::core::default::Default>::default()
            }
        }
        impl<'a> ::ink::codegen::StaticEnv for Recursive {
            type EnvAccess = ::ink::EnvAccess<
                'static,
                <Recursive as ::ink::env::ContractEnv>::Env,
            >;
            fn env() -> Self::EnvAccess {
                <<Self as ::ink::codegen::StaticEnv>::EnvAccess as ::core::default::Default>::default()
            }
        }
    };
    const _: () = {
        #[allow(unused_imports)]
        use ::ink::codegen::{Env as _, StaticEnv as _};
        use ::ink::codegen::EmitEvent as _;
    };
    const _: () = {
        impl<'a> ::ink::codegen::EmitEvent<Recursive>
        for ::ink::EnvAccess<'a, Environment> {
            fn emit_event<E>(self, event: E)
            where
                E: Into<<Recursive as ::ink::reflect::ContractEventBase>::Type>,
            {
                ::ink::env::emit_event::<
                    Environment,
                    <Recursive as ::ink::reflect::ContractEventBase>::Type,
                >(event.into());
            }
        }
    };
    #[allow(non_camel_case_types)]
    #[cfg(not(feature = "__ink_dylint_EventBase"))]
    pub enum __ink_EventBase {
        CurrentValue(CurrentValue),
    }
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::scale::Encode for __ink_EventBase {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        __ink_EventBase::CurrentValue(ref aa) => {
                            0_usize.saturating_add(::scale::Encode::size_hint(aa))
                        }
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    __ink_EventBase::CurrentValue(ref aa) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::scale::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for __ink_EventBase {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::scale::Decode for __ink_EventBase {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e
                            .chain(
                                "Could not decode `__ink_EventBase`, failed to read variant byte",
                            )
                    })?
                {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(
                                __ink_EventBase::CurrentValue({
                                    let __codec_res_edqy = <CurrentValue as ::scale::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e
                                                    .chain("Could not decode `__ink_EventBase::CurrentValue.0`"),
                                            );
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                }),
                            )
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(
                                <_ as ::core::convert::Into<
                                    _,
                                >>::into(
                                    "Could not decode `__ink_EventBase`, variant doesn't exist",
                                ),
                            )
                        })();
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink::reflect::ContractEventBase for Recursive {
            type Type = __ink_EventBase;
        }
    };
    const _: () = {
        impl From<CurrentValue> for __ink_EventBase {
            fn from(event: CurrentValue) -> Self {
                Self::CurrentValue(event)
            }
        }
    };
    const _: () = {
        pub enum __ink_UndefinedAmountOfTopics {}
        impl ::ink::env::topics::EventTopicsAmount for __ink_UndefinedAmountOfTopics {
            const AMOUNT: usize = 0;
        }
        impl ::ink::env::Topics for __ink_EventBase {
            type RemainingTopics = __ink_UndefinedAmountOfTopics;
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<
                    ::ink::env::topics::state::Uninit,
                    E,
                    B,
                >,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                match self {
                    Self::CurrentValue(event) => {
                        <CurrentValue as ::ink::env::Topics>::topics::<
                            E,
                            B,
                        >(event, builder)
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!("Event does not exist!"),
                        );
                    }
                }
            }
        }
    };
    impl ::ink::codegen::EventLenTopics for CurrentValue {
        type LenTopics = ::ink::codegen::EventTopics<0usize>;
    }
    const _: () = ::ink::codegen::utils::consume_type::<
        ::ink::codegen::EventRespectsTopicLimit<
            CurrentValue,
            {
                <<Recursive as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::MAX_EVENT_TOPICS
            },
        >,
    >();
    pub struct CurrentValue {
        status: u64,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for CurrentValue {
            fn size_hint(&self) -> usize {
                ::scale::Encode::size_hint(&&self.status)
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.status, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.status)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.status, f)
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for CurrentValue {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for CurrentValue {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(CurrentValue {
                    status: {
                        let __codec_res_edqy = <u64 as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `CurrentValue::status`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    const _: () = {
        impl ::ink::env::Topics for CurrentValue {
            type RemainingTopics = [::ink::env::topics::state::HasRemainingTopics; 1usize];
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<
                    ::ink::env::topics::state::Uninit,
                    E,
                    B,
                >,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                builder
                    .build::<Self>()
                    .push_topic::<
                        ::ink::env::topics::PrefixedValue<[u8; 23usize]>,
                    >(
                        &::ink::env::topics::PrefixedValue {
                            value: b"Recursive::CurrentValue",
                            prefix: b"",
                        },
                    )
                    .finish()
            }
        }
    };
    impl ::ink::reflect::DispatchableConstructorInfo<0x9BAE9D5E_u32> for Recursive {
        type Input = u64;
        type Output = Self;
        type Storage = Recursive;
        type Error = <::ink::reflect::ConstructorOutputValue<
            Self,
        > as ::ink::reflect::ConstructorOutput<Recursive>>::Error;
        const IS_RESULT: ::core::primitive::bool = <::ink::reflect::ConstructorOutputValue<
            Self,
        > as ::ink::reflect::ConstructorOutput<Recursive>>::IS_RESULT;
        const CALLABLE: fn(Self::Input) -> Self::Output = |__ink_binding_0| {
            Recursive::new(__ink_binding_0)
        };
        const PAYABLE: ::core::primitive::bool = false;
        const SELECTOR: [::core::primitive::u8; 4usize] = [
            0x9B_u8,
            0xAE_u8,
            0x9D_u8,
            0x5E_u8,
        ];
        const LABEL: &'static ::core::primitive::str = "new";
    }
    impl ::ink::reflect::DispatchableMessageInfo<
        {
            ::core::primitive::u32::from_be_bytes({
                <<::ink::reflect::TraitDefinitionRegistry<
                    <Recursive as ::ink::env::ContractEnv>::Env,
                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                    0x12BD51D3_u32,
                >>::SELECTOR
            })
        },
    > for Recursive {
        type Input = ();
        type Output = ();
        type Storage = Recursive;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { <Recursive as RecursiveCall>::increment(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = {
            <<::ink::reflect::TraitDefinitionRegistry<
                <Recursive as ::ink::env::ContractEnv>::Env,
            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                0x12BD51D3_u32,
            >>::SELECTOR
        };
        const PAYABLE: ::core::primitive::bool = {
            <<::ink::reflect::TraitDefinitionRegistry<
                <Recursive as ::ink::env::ContractEnv>::Env,
            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                0x12BD51D3_u32,
            >>::PAYABLE
        };
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "RecursiveCall::increment";
    }
    impl ::ink::reflect::DispatchableMessageInfo<
        {
            ::core::primitive::u32::from_be_bytes({
                <<::ink::reflect::TraitDefinitionRegistry<
                    <Recursive as ::ink::env::ContractEnv>::Env,
                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                    0x7F9EFB98_u32,
                >>::SELECTOR
            })
        },
    > for Recursive {
        type Input = ();
        type Output = ();
        type Storage = Recursive;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { <Recursive as RecursiveCall>::increment_recursive(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = {
            <<::ink::reflect::TraitDefinitionRegistry<
                <Recursive as ::ink::env::ContractEnv>::Env,
            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                0x7F9EFB98_u32,
            >>::SELECTOR
        };
        const PAYABLE: ::core::primitive::bool = {
            <<::ink::reflect::TraitDefinitionRegistry<
                <Recursive as ::ink::env::ContractEnv>::Env,
            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                0x7F9EFB98_u32,
            >>::PAYABLE
        };
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "RecursiveCall::increment_recursive";
    }
    impl ::ink::reflect::DispatchableMessageInfo<
        {
            ::core::primitive::u32::from_be_bytes({
                <<::ink::reflect::TraitDefinitionRegistry<
                    <Recursive as ::ink::env::ContractEnv>::Env,
                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                    0x2F865BD9_u32,
                >>::SELECTOR
            })
        },
    > for Recursive {
        type Input = ();
        type Output = u64;
        type Storage = Recursive;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output = |
            storage,
            _|
        { <Recursive as RecursiveCall>::get(storage) };
        const SELECTOR: [::core::primitive::u8; 4usize] = {
            <<::ink::reflect::TraitDefinitionRegistry<
                <Recursive as ::ink::env::ContractEnv>::Env,
            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                0x2F865BD9_u32,
            >>::SELECTOR
        };
        const PAYABLE: ::core::primitive::bool = {
            <<::ink::reflect::TraitDefinitionRegistry<
                <Recursive as ::ink::env::ContractEnv>::Env,
            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                0x2F865BD9_u32,
            >>::PAYABLE
        };
        const MUTATES: ::core::primitive::bool = false;
        const LABEL: &'static ::core::primitive::str = "RecursiveCall::get";
    }
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_ConstructorDecoder {
            Constructor0(
                <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                    0x9BAE9D5E_u32,
                >>::Input,
            ),
        }
        impl ::ink::reflect::DecodeDispatch for __ink_ConstructorDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::reflect::DispatchError>
            where
                I: ::scale::Input,
            {
                const CONSTRUCTOR_0: [::core::primitive::u8; 4usize] = <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                    0x9BAE9D5E_u32,
                >>::SELECTOR;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::reflect::DispatchError::InvalidSelector)?
                {
                    CONSTRUCTOR_0 => {
                        ::core::result::Result::Ok(
                            Self::Constructor0(
                                <<Recursive as ::ink::reflect::DispatchableConstructorInfo<
                                    0x9BAE9D5E_u32,
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    _invalid => {
                        ::core::result::Result::Err(
                            ::ink::reflect::DispatchError::UnknownSelector,
                        )
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_ConstructorDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::reflect::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_ConstructorDecoder {
            #[allow(clippy::nonminimal_bool)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::reflect::DispatchError> {
                match self {
                    Self::Constructor0(input) => {
                        if {
                            false
                                || {
                                    let constructor_0 = false;
                                    let constructor_0 = <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                                        0x9BAE9D5E_u32,
                                    >>::PAYABLE;
                                    constructor_0
                                }
                        }
                            && !<Recursive as ::ink::reflect::DispatchableConstructorInfo<
                                0x9BAE9D5E_u32,
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::Output = <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::CALLABLE(input);
                        let output_value = ::ink::reflect::ConstructorOutputValue::new(
                            result,
                        );
                        let output_result = <::ink::reflect::ConstructorOutputValue<
                            <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                                0x9BAE9D5E_u32,
                            >>::Output,
                        > as ::ink::reflect::ConstructorOutput<
                            Recursive,
                        >>::as_result(&output_value);
                        if let ::core::result::Result::Ok(contract)
                            = output_result.as_ref()
                        {
                            ::ink::env::set_contract_storage::<
                                ::ink::primitives::Key,
                                Recursive,
                            >(
                                &<Recursive as ::ink::storage::traits::StorageKey>::KEY,
                                contract,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::ConstructorResult<
                                ::core::result::Result<
                                    (),
                                    &<::ink::reflect::ConstructorOutputValue<
                                        <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                                            0x9BAE9D5E_u32,
                                        >>::Output,
                                    > as ::ink::reflect::ConstructorOutput<Recursive>>::Error,
                                >,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(
                                output_result.is_err(),
                            ),
                            &::ink::ConstructorResult::Ok(output_result.map(|_| ())),
                        );
                    }
                }
            }
        }
        impl ::ink::reflect::ContractConstructorDecoder for Recursive {
            type Type = __ink_ConstructorDecoder;
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_MessageDecoder {
            Message0(
                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                    {
                        ::core::primitive::u32::from_be_bytes(
                            <<::ink::reflect::TraitDefinitionRegistry<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                0x12BD51D3_u32,
                            >>::SELECTOR,
                        )
                    },
                >>::Input,
            ),
            Message1(
                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                    {
                        ::core::primitive::u32::from_be_bytes(
                            <<::ink::reflect::TraitDefinitionRegistry<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                0x7F9EFB98_u32,
                            >>::SELECTOR,
                        )
                    },
                >>::Input,
            ),
            Message2(
                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                    {
                        ::core::primitive::u32::from_be_bytes(
                            <<::ink::reflect::TraitDefinitionRegistry<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                0x2F865BD9_u32,
                            >>::SELECTOR,
                        )
                    },
                >>::Input,
            ),
        }
        impl ::ink::reflect::DecodeDispatch for __ink_MessageDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::reflect::DispatchError>
            where
                I: ::scale::Input,
            {
                const MESSAGE_0: [::core::primitive::u8; 4usize] = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                    {
                        ::core::primitive::u32::from_be_bytes(
                            <<::ink::reflect::TraitDefinitionRegistry<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                0x12BD51D3_u32,
                            >>::SELECTOR,
                        )
                    },
                >>::SELECTOR;
                const MESSAGE_1: [::core::primitive::u8; 4usize] = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                    {
                        ::core::primitive::u32::from_be_bytes(
                            <<::ink::reflect::TraitDefinitionRegistry<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                0x7F9EFB98_u32,
                            >>::SELECTOR,
                        )
                    },
                >>::SELECTOR;
                const MESSAGE_2: [::core::primitive::u8; 4usize] = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                    {
                        ::core::primitive::u32::from_be_bytes(
                            <<::ink::reflect::TraitDefinitionRegistry<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                0x2F865BD9_u32,
                            >>::SELECTOR,
                        )
                    },
                >>::SELECTOR;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::reflect::DispatchError::InvalidSelector)?
                {
                    MESSAGE_0 => {
                        ::core::result::Result::Ok(
                            Self::Message0(
                                <<Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x12BD51D3_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    MESSAGE_1 => {
                        ::core::result::Result::Ok(
                            Self::Message1(
                                <<Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x7F9EFB98_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    MESSAGE_2 => {
                        ::core::result::Result::Ok(
                            Self::Message2(
                                <<Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x2F865BD9_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Input as ::scale::Decode>::decode(input)
                                    .map_err(|_| {
                                        ::ink::reflect::DispatchError::InvalidParameters
                                    })?,
                            ),
                        )
                    }
                    _invalid => {
                        ::core::result::Result::Err(
                            ::ink::reflect::DispatchError::UnknownSelector,
                        )
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_MessageDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::reflect::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        fn push_contract(contract: ::core::mem::ManuallyDrop<Recursive>, mutates: bool) {
            if mutates {
                ::ink::env::set_contract_storage::<
                    ::ink::primitives::Key,
                    Recursive,
                >(&<Recursive as ::ink::storage::traits::StorageKey>::KEY, &contract);
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_MessageDecoder {
            #[allow(clippy::nonminimal_bool, clippy::let_unit_value)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::reflect::DispatchError> {
                let key = <Recursive as ::ink::storage::traits::StorageKey>::KEY;
                let mut contract: ::core::mem::ManuallyDrop<Recursive> = ::core::mem::ManuallyDrop::new(
                    match ::ink::env::get_contract_storage(&key) {
                        ::core::result::Result::Ok(
                            ::core::option::Option::Some(value),
                        ) => value,
                        ::core::result::Result::Ok(::core::option::Option::None) => {
                            ::core::panicking::panic_fmt(
                                format_args!("storage entry was empty"),
                            );
                        }
                        ::core::result::Result::Err(_) => {
                            ::core::panicking::panic_fmt(
                                format_args!("could not properly decode storage entry"),
                            );
                        }
                    },
                );
                match self {
                    Self::Message0(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x12BD51D3_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x7F9EFB98_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x2F865BD9_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_2
                                }
                        }
                            && !<Recursive as ::ink::reflect::DispatchableMessageInfo<
                                {
                                    ::core::primitive::u32::from_be_bytes(
                                        <<::ink::reflect::TraitDefinitionRegistry<
                                            <Recursive as ::ink::env::ContractEnv>::Env,
                                        > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                            0x12BD51D3_u32,
                                        >>::SELECTOR,
                                    )
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Recursive as ::ink::reflect::DispatchableMessageInfo<
                            {
                                ::core::primitive::u32::from_be_bytes(
                                    <<::ink::reflect::TraitDefinitionRegistry<
                                        <Recursive as ::ink::env::ContractEnv>::Env,
                                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                        0x12BD51D3_u32,
                                    >>::SELECTOR,
                                )
                            },
                        >>::Output = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                            {
                                ::core::primitive::u32::from_be_bytes(
                                    <<::ink::reflect::TraitDefinitionRegistry<
                                        <Recursive as ::ink::env::ContractEnv>::Env,
                                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                        0x12BD51D3_u32,
                                    >>::SELECTOR,
                                )
                            },
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x12BD51D3_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x12BD51D3_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x12BD51D3_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message1(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x12BD51D3_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x7F9EFB98_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x2F865BD9_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_2
                                }
                        }
                            && !<Recursive as ::ink::reflect::DispatchableMessageInfo<
                                {
                                    ::core::primitive::u32::from_be_bytes(
                                        <<::ink::reflect::TraitDefinitionRegistry<
                                            <Recursive as ::ink::env::ContractEnv>::Env,
                                        > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                            0x7F9EFB98_u32,
                                        >>::SELECTOR,
                                    )
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Recursive as ::ink::reflect::DispatchableMessageInfo<
                            {
                                ::core::primitive::u32::from_be_bytes(
                                    <<::ink::reflect::TraitDefinitionRegistry<
                                        <Recursive as ::ink::env::ContractEnv>::Env,
                                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                        0x7F9EFB98_u32,
                                    >>::SELECTOR,
                                )
                            },
                        >>::Output = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                            {
                                ::core::primitive::u32::from_be_bytes(
                                    <<::ink::reflect::TraitDefinitionRegistry<
                                        <Recursive as ::ink::env::ContractEnv>::Env,
                                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                        0x7F9EFB98_u32,
                                    >>::SELECTOR,
                                )
                            },
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x7F9EFB98_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x7F9EFB98_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x7F9EFB98_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message2(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x12BD51D3_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x7F9EFB98_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                        {
                                            ::core::primitive::u32::from_be_bytes(
                                                <<::ink::reflect::TraitDefinitionRegistry<
                                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                    0x2F865BD9_u32,
                                                >>::SELECTOR,
                                            )
                                        },
                                    >>::PAYABLE;
                                    message_2
                                }
                        }
                            && !<Recursive as ::ink::reflect::DispatchableMessageInfo<
                                {
                                    ::core::primitive::u32::from_be_bytes(
                                        <<::ink::reflect::TraitDefinitionRegistry<
                                            <Recursive as ::ink::env::ContractEnv>::Env,
                                        > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                            0x2F865BD9_u32,
                                        >>::SELECTOR,
                                    )
                                },
                            >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <Recursive as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <Recursive as ::ink::reflect::DispatchableMessageInfo<
                            {
                                ::core::primitive::u32::from_be_bytes(
                                    <<::ink::reflect::TraitDefinitionRegistry<
                                        <Recursive as ::ink::env::ContractEnv>::Env,
                                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                        0x2F865BD9_u32,
                                    >>::SELECTOR,
                                )
                            },
                        >>::Output = <Recursive as ::ink::reflect::DispatchableMessageInfo<
                            {
                                ::core::primitive::u32::from_be_bytes(
                                    <<::ink::reflect::TraitDefinitionRegistry<
                                        <Recursive as ::ink::env::ContractEnv>::Env,
                                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                        0x2F865BD9_u32,
                                    >>::SELECTOR,
                                )
                            },
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x2F865BD9_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Output,
                            >::VALUE
                        }
                            && {
                                #[allow(unused_imports)]
                                use ::ink::result_info::IsResultErrFallback as _;
                                ::ink::result_info::IsResultErr(&result).value()
                            };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x2F865BD9_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <Recursive as ::ink::reflect::DispatchableMessageInfo<
                                    {
                                        ::core::primitive::u32::from_be_bytes(
                                            <<::ink::reflect::TraitDefinitionRegistry<
                                                <Recursive as ::ink::env::ContractEnv>::Env,
                                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                                0x2F865BD9_u32,
                                            >>::SELECTOR,
                                        )
                                    },
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                };
            }
        }
        impl ::ink::reflect::ContractMessageDecoder for Recursive {
            type Type = __ink_MessageDecoder;
        }
    };
    const _: () = {
        use ::ink::codegen::{Env as _, StaticEnv as _};
        use ::ink::codegen::EmitEvent as _;
        const _: ::ink::codegen::utils::IsSameType<Recursive> = ::ink::codegen::utils::IsSameType::<
            Recursive,
        >::new();
        impl RecursiveCall for Recursive {
            type __ink_TraitInfo = <::ink::reflect::TraitDefinitionRegistry<
                Environment,
            > as RecursiveCall>::__ink_TraitInfo;
            type incrementOutput = ();
            fn increment(&mut self) -> Self::incrementOutput {
                self.env().emit_event(CurrentValue { status: self.value });
            }
            type incrementRecursiveOutput = ();
            fn increment_recursive(&mut self) -> Self::incrementRecursiveOutput {}
            type getOutput = u64;
            fn get(&self) -> Self::getOutput {
                self.value
            }
        }
        const _: ::ink::codegen::utils::IsSameType<Recursive> = ::ink::codegen::utils::IsSameType::<
            Recursive,
        >::new();
        impl Recursive {
            #[cfg(not(feature = "__ink_dylint_Constructor"))]
            pub fn new(init_value: u64) -> Self {
                Self { value: init_value }
            }
        }
        const _: () = {
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u64>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<u64>>();
        };
    };
    const _: () = {
        /// The ink! smart contract's call builder.
        ///
        /// Implements the underlying on-chain calling of the ink! smart contract
        /// messages and trait implementations in a type safe way.
        #[repr(transparent)]
        pub struct CallBuilder {
            account_id: AccountId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CallBuilder {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CallBuilder",
                    "account_id",
                    &&self.account_id,
                )
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Encode for CallBuilder {
                fn size_hint(&self) -> usize {
                    ::scale::Encode::size_hint(&&self.account_id)
                }
                fn encode_to<
                    __CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized,
                >(&self, __codec_dest_edqy: &mut __CodecOutputEdqy) {
                    ::scale::Encode::encode_to(&&self.account_id, __codec_dest_edqy)
                }
                fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                    ::scale::Encode::encode(&&self.account_id)
                }
                fn using_encoded<
                    R,
                    F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R,
                >(&self, f: F) -> R {
                    ::scale::Encode::using_encoded(&&self.account_id, f)
                }
            }
            #[automatically_derived]
            impl ::scale::EncodeLike for CallBuilder {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Decode for CallBuilder {
                fn decode<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    ::core::result::Result::Ok(CallBuilder {
                        account_id: {
                            let __codec_res_edqy = <AccountId as ::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `CallBuilder::account_id`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => {
                                    __codec_res_edqy
                                }
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::scale::DecodeFinished, ::scale::Error> {
                    match (
                        &::core::mem::size_of::<AccountId>(),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<AccountId>() > 0 { 1 } else { 0 }
                        <= 1)
                    {
                        ::core::panicking::panic(
                            "assertion failed: if ::core::mem::size_of::<AccountId>() > 0 { 1 } else { 0 } <= 1",
                        )
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<AccountId> = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<::core::mem::MaybeUninit<AccountId>>()
                        };
                        <AccountId as ::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[automatically_derived]
        impl ::core::hash::Hash for CallBuilder {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.account_id, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CallBuilder {
            #[inline]
            fn eq(&self, other: &CallBuilder) -> bool {
                self.account_id == other.account_id
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CallBuilder {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CallBuilder {
            #[inline]
            fn clone(&self) -> CallBuilder {
                CallBuilder {
                    account_id: ::core::clone::Clone::clone(&self.account_id),
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for CallBuilder {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(
                            ::scale_info::Path::new(
                                "CallBuilder",
                                "recursive::recursive",
                            ),
                        )
                        .type_params(::alloc::vec::Vec::new())
                        .docs(
                            &[
                                "The ink! smart contract's call builder.",
                                "",
                                "Implements the underlying on-chain calling of the ink! smart contract",
                                "messages and trait implementations in a type safe way.",
                            ],
                        )
                        .composite(
                            ::scale_info::build::Fields::named()
                                .field(|f| {
                                    f
                                        .ty::<AccountId>()
                                        .name("account_id")
                                        .type_name("AccountId")
                                }),
                        )
                }
            }
        };
        const _: () = {
            impl ::ink::storage::traits::StorageLayout for CallBuilder {
                fn layout(
                    __key: &::ink::primitives::Key,
                ) -> ::ink::metadata::layout::Layout {
                    ::ink::metadata::layout::Layout::Struct(
                        ::ink::metadata::layout::StructLayout::new(
                            "CallBuilder",
                            [
                                ::ink::metadata::layout::FieldLayout::new(
                                    "account_id",
                                    <AccountId as ::ink::storage::traits::StorageLayout>::layout(
                                        __key,
                                    ),
                                ),
                            ],
                        ),
                    )
                }
            }
        };
        const _: () = {
            impl ::ink::codegen::ContractCallBuilder for Recursive {
                type Type = CallBuilder;
            }
            impl ::ink::env::ContractEnv for CallBuilder {
                type Env = <Recursive as ::ink::env::ContractEnv>::Env;
            }
        };
        impl ::ink::env::call::FromAccountId<Environment> for CallBuilder {
            #[inline]
            fn from_account_id(account_id: AccountId) -> Self {
                Self { account_id }
            }
        }
        impl ::ink::ToAccountId<Environment> for CallBuilder {
            #[inline]
            fn to_account_id(&self) -> AccountId {
                <AccountId as ::core::clone::Clone>::clone(&self.account_id)
            }
        }
        impl ::core::convert::AsRef<AccountId> for CallBuilder {
            fn as_ref(&self) -> &AccountId {
                &self.account_id
            }
        }
        impl ::core::convert::AsMut<AccountId> for CallBuilder {
            fn as_mut(&mut self) -> &mut AccountId {
                &mut self.account_id
            }
        }
        #[doc(hidden)]
        impl ::ink::codegen::TraitCallForwarderFor<
            {
                <<::ink::reflect::TraitDefinitionRegistry<
                    Environment,
                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
            },
        > for CallBuilder {
            type Forwarder = <<Self as RecursiveCall>::__ink_TraitInfo as ::ink::codegen::TraitCallForwarder>::Forwarder;
            #[inline]
            fn forward(&self) -> &Self::Forwarder {
                unsafe {
                    &*(&self.account_id as *const AccountId as *const Self::Forwarder)
                }
            }
            #[inline]
            fn forward_mut(&mut self) -> &mut Self::Forwarder {
                unsafe {
                    &mut *(&mut self.account_id as *mut AccountId
                        as *mut Self::Forwarder)
                }
            }
            #[inline]
            fn build(
                &self,
            ) -> &<Self::Forwarder as ::ink::codegen::TraitCallBuilder>::Builder {
                <_ as ::ink::codegen::TraitCallBuilder>::call(
                    <Self as ::ink::codegen::TraitCallForwarderFor<
                        {
                            <<::ink::reflect::TraitDefinitionRegistry<
                                Environment,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                        },
                    >>::forward(self),
                )
            }
            #[inline]
            fn build_mut(
                &mut self,
            ) -> &mut <Self::Forwarder as ::ink::codegen::TraitCallBuilder>::Builder {
                <_ as ::ink::codegen::TraitCallBuilder>::call_mut(
                    <Self as ::ink::codegen::TraitCallForwarderFor<
                        {
                            <<::ink::reflect::TraitDefinitionRegistry<
                                Environment,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                        },
                    >>::forward_mut(self),
                )
            }
        }
        impl RecursiveCall for CallBuilder {
            type __ink_TraitInfo = <::ink::reflect::TraitDefinitionRegistry<
                Environment,
            > as RecursiveCall>::__ink_TraitInfo;
            type incrementOutput = <<<Self as ::ink::codegen::TraitCallForwarderFor<
                {
                    <<::ink::reflect::TraitDefinitionRegistry<
                        Environment,
                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                },
            >>::Forwarder as ::ink::codegen::TraitCallBuilder>::Builder as RecursiveCall>::incrementOutput;
            #[inline]
            fn increment(&mut self) -> Self::incrementOutput {
                <_ as RecursiveCall>::increment(
                    <Self as ::ink::codegen::TraitCallForwarderFor<
                        {
                            <<::ink::reflect::TraitDefinitionRegistry<
                                Environment,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                        },
                    >>::build_mut(self),
                )
            }
            type incrementRecursiveOutput = <<<Self as ::ink::codegen::TraitCallForwarderFor<
                {
                    <<::ink::reflect::TraitDefinitionRegistry<
                        Environment,
                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                },
            >>::Forwarder as ::ink::codegen::TraitCallBuilder>::Builder as RecursiveCall>::incrementRecursiveOutput;
            #[inline]
            fn increment_recursive(&mut self) -> Self::incrementRecursiveOutput {
                <_ as RecursiveCall>::increment_recursive(
                    <Self as ::ink::codegen::TraitCallForwarderFor<
                        {
                            <<::ink::reflect::TraitDefinitionRegistry<
                                Environment,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                        },
                    >>::build_mut(self),
                )
            }
            type getOutput = <<<Self as ::ink::codegen::TraitCallForwarderFor<
                {
                    <<::ink::reflect::TraitDefinitionRegistry<
                        Environment,
                    > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                },
            >>::Forwarder as ::ink::codegen::TraitCallBuilder>::Builder as RecursiveCall>::getOutput;
            #[inline]
            fn get(&self) -> Self::getOutput {
                <_ as RecursiveCall>::get(
                    <Self as ::ink::codegen::TraitCallForwarderFor<
                        {
                            <<::ink::reflect::TraitDefinitionRegistry<
                                Environment,
                            > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                        },
                    >>::build(self),
                )
            }
        }
        impl CallBuilder {}
    };
    pub struct RecursiveRef {
        inner: <Recursive as ::ink::codegen::ContractCallBuilder>::Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RecursiveRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "RecursiveRef",
                "inner",
                &&self.inner,
            )
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for RecursiveRef {
            fn size_hint(&self) -> usize {
                ::scale::Encode::size_hint(&&self.inner)
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.inner, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.inner)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.inner, f)
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for RecursiveRef {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for RecursiveRef {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(RecursiveRef {
                    inner: {
                        let __codec_res_edqy = <<Recursive as ::ink::codegen::ContractCallBuilder>::Type as ::scale::Decode>::decode(
                            __codec_input_edqy,
                        );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `RecursiveRef::inner`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => {
                                __codec_res_edqy
                            }
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::hash::Hash for RecursiveRef {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RecursiveRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RecursiveRef {
        #[inline]
        fn eq(&self, other: &RecursiveRef) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for RecursiveRef {}
    #[automatically_derived]
    impl ::core::cmp::Eq for RecursiveRef {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Recursive as ::ink::codegen::ContractCallBuilder>::Type,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RecursiveRef {
        #[inline]
        fn clone(&self) -> RecursiveRef {
            RecursiveRef {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for RecursiveRef {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(
                        ::scale_info::Path::new("RecursiveRef", "recursive::recursive"),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <Recursive as ::ink::codegen::ContractCallBuilder>::Type,
                                    >()
                                    .name("inner")
                                    .type_name(
                                        "<Recursive as::ink::codegen::ContractCallBuilder>::Type",
                                    )
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for RecursiveRef {
            fn layout(
                __key: &::ink::primitives::Key,
            ) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "RecursiveRef",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "inner",
                                <<Recursive as ::ink::codegen::ContractCallBuilder>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            ),
                        ],
                    ),
                )
            }
        }
    };
    const _: () = {
        impl ::ink::env::ContractReference for Recursive {
            type Type = RecursiveRef;
        }
        impl ::ink::env::call::ConstructorReturnType<RecursiveRef> for Recursive {
            type Output = RecursiveRef;
            type Error = ();
            fn ok(value: RecursiveRef) -> Self::Output {
                value
            }
        }
        impl<E> ::ink::env::call::ConstructorReturnType<RecursiveRef>
        for ::core::result::Result<Recursive, E>
        where
            E: ::scale::Decode,
        {
            const IS_RESULT: bool = true;
            type Output = ::core::result::Result<RecursiveRef, E>;
            type Error = E;
            fn ok(value: RecursiveRef) -> Self::Output {
                ::core::result::Result::Ok(value)
            }
            fn err(err: Self::Error) -> ::core::option::Option<Self::Output> {
                ::core::option::Option::Some(::core::result::Result::Err(err))
            }
        }
        impl ::ink::env::ContractEnv for RecursiveRef {
            type Env = <Recursive as ::ink::env::ContractEnv>::Env;
        }
    };
    impl RecursiveCall for RecursiveRef {
        type __ink_TraitInfo = <::ink::reflect::TraitDefinitionRegistry<
            Environment,
        > as RecursiveCall>::__ink_TraitInfo;
        type incrementOutput = <<Self::__ink_TraitInfo as ::ink::codegen::TraitCallForwarder>::Forwarder as RecursiveCall>::incrementOutput;
        #[inline]
        fn increment(&mut self) -> Self::incrementOutput {
            <_ as RecursiveCall>::increment(
                <_ as ::ink::codegen::TraitCallForwarderFor<
                    {
                        <<::ink::reflect::TraitDefinitionRegistry<
                            Environment,
                        > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                    },
                >>::forward_mut(
                    <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self),
                ),
            )
        }
        type incrementRecursiveOutput = <<Self::__ink_TraitInfo as ::ink::codegen::TraitCallForwarder>::Forwarder as RecursiveCall>::incrementRecursiveOutput;
        #[inline]
        fn increment_recursive(&mut self) -> Self::incrementRecursiveOutput {
            <_ as RecursiveCall>::increment_recursive(
                <_ as ::ink::codegen::TraitCallForwarderFor<
                    {
                        <<::ink::reflect::TraitDefinitionRegistry<
                            Environment,
                        > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                    },
                >>::forward_mut(
                    <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self),
                ),
            )
        }
        type getOutput = <<Self::__ink_TraitInfo as ::ink::codegen::TraitCallForwarder>::Forwarder as RecursiveCall>::getOutput;
        #[inline]
        fn get(&self) -> Self::getOutput {
            <_ as RecursiveCall>::get(
                <_ as ::ink::codegen::TraitCallForwarderFor<
                    {
                        <<::ink::reflect::TraitDefinitionRegistry<
                            Environment,
                        > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitInfo>::ID
                    },
                >>::forward(<Self as ::ink::codegen::TraitCallBuilder>::call(self)),
            )
        }
    }
    impl RecursiveRef {
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new(
            __ink_binding_0: u64,
        ) -> ::ink::env::call::CreateBuilder<
            Environment,
            Self,
            ::ink::env::call::utils::Unset<Hash>,
            ::ink::env::call::utils::Unset<u64>,
            ::ink::env::call::utils::Unset<Balance>,
            ::ink::env::call::utils::Set<
                ::ink::env::call::ExecutionInput<
                    ::ink::env::call::utils::ArgumentList<
                        ::ink::env::call::utils::Argument<u64>,
                        ::ink::env::call::utils::EmptyArgumentList,
                    >,
                >,
            >,
            ::ink::env::call::utils::Unset<::ink::env::call::state::Salt>,
            ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<Self>>,
        > {
            ::ink::env::call::build_create::<Self>()
                .exec_input(
                    ::ink::env::call::ExecutionInput::new(
                            ::ink::env::call::Selector::new([
                                0x9B_u8,
                                0xAE_u8,
                                0x9D_u8,
                                0x5E_u8,
                            ]),
                        )
                        .push_arg(__ink_binding_0),
                )
                .returns::<Self>()
        }
    }
    const _: () = {
        impl ::ink::codegen::TraitCallBuilder for RecursiveRef {
            type Builder = <Recursive as ::ink::codegen::ContractCallBuilder>::Type;
            #[inline]
            fn call(&self) -> &Self::Builder {
                &self.inner
            }
            #[inline]
            fn call_mut(&mut self) -> &mut Self::Builder {
                &mut self.inner
            }
        }
    };
    impl ::ink::env::call::FromAccountId<Environment> for RecursiveRef {
        #[inline]
        fn from_account_id(account_id: AccountId) -> Self {
            Self {
                inner: <<Recursive as ::ink::codegen::ContractCallBuilder>::Type as ::ink::env::call::FromAccountId<
                    Environment,
                >>::from_account_id(account_id),
            }
        }
    }
    impl ::ink::ToAccountId<Environment> for RecursiveRef {
        #[inline]
        fn to_account_id(&self) -> AccountId {
            <<Recursive as ::ink::codegen::ContractCallBuilder>::Type as ::ink::ToAccountId<
                Environment,
            >>::to_account_id(&self.inner)
        }
    }
    impl ::core::convert::AsRef<AccountId> for RecursiveRef {
        fn as_ref(&self) -> &AccountId {
            <_ as ::core::convert::AsRef<AccountId>>::as_ref(&self.inner)
        }
    }
    impl ::core::convert::AsMut<AccountId> for RecursiveRef {
        fn as_mut(&mut self) -> &mut AccountId {
            <_ as ::core::convert::AsMut<AccountId>>::as_mut(&mut self.inner)
        }
    }
    #[cfg(feature = "std")]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[no_mangle]
        pub fn __ink_generate_metadata() -> ::ink::metadata::InkProject {
            let layout = ::ink::metadata::layout::Layout::Root(
                ::ink::metadata::layout::RootLayout::new(
                    <::ink::metadata::layout::LayoutKey as ::core::convert::From<
                        ::ink::primitives::Key,
                    >>::from(<Recursive as ::ink::storage::traits::StorageKey>::KEY),
                    <Recursive as ::ink::storage::traits::StorageLayout>::layout(
                        &<Recursive as ::ink::storage::traits::StorageKey>::KEY,
                    ),
                ),
            );
            ::ink::metadata::layout::ValidateLayout::validate(&layout)
                .unwrap_or_else(|error| {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("metadata ink! generation failed: {0}", error),
                        );
                    }
                });
            ::ink::metadata::InkProject::new(
                layout,
                ::ink::metadata::ContractSpec::new()
                    .constructors([
                        ::ink::metadata::ConstructorSpec::from_label("new")
                            .selector([0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8])
                            .args([
                                ::ink::metadata::MessageParamSpec::new("init_value")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u64,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u64"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                            ])
                            .payable(false)
                            .default(false)
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    if <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                                        2611912030u32,
                                    >>::IS_RESULT {
                                        ::core::option::Option::Some(
                                            ::ink::metadata::TypeSpec::with_name_str::<
                                                ::ink::ConstructorResult<
                                                    ::core::result::Result<
                                                        (),
                                                        <Recursive as ::ink::reflect::DispatchableConstructorInfo<
                                                            2611912030u32,
                                                        >>::Error,
                                                    >,
                                                >,
                                            >("ink_primitives::ConstructorResult"),
                                        )
                                    } else {
                                        ::core::option::Option::Some(
                                            ::ink::metadata::TypeSpec::with_name_str::<
                                                ::ink::ConstructorResult<()>,
                                            >("ink_primitives::ConstructorResult"),
                                        )
                                    },
                                ),
                            )
                            .docs([])
                            .done(),
                    ])
                    .messages([
                        ::ink::metadata::MessageSpec::from_label(
                                "RecursiveCall::increment",
                            )
                            .selector({
                                <<::ink::reflect::TraitDefinitionRegistry<
                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                    0x12BD51D3_u32,
                                >>::SELECTOR
                            })
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<()>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(true)
                            .payable({
                                <<::ink::reflect::TraitDefinitionRegistry<
                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                    0x12BD51D3_u32,
                                >>::PAYABLE
                            })
                            .docs([])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label(
                                "RecursiveCall::increment_recursive",
                            )
                            .selector({
                                <<::ink::reflect::TraitDefinitionRegistry<
                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                    0x7F9EFB98_u32,
                                >>::SELECTOR
                            })
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<()>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(true)
                            .payable({
                                <<::ink::reflect::TraitDefinitionRegistry<
                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                    0x7F9EFB98_u32,
                                >>::PAYABLE
                            })
                            .docs([])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("RecursiveCall::get")
                            .selector({
                                <<::ink::reflect::TraitDefinitionRegistry<
                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                    0x2F865BD9_u32,
                                >>::SELECTOR
                            })
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<u64>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(false)
                            .payable({
                                <<::ink::reflect::TraitDefinitionRegistry<
                                    <Recursive as ::ink::env::ContractEnv>::Env,
                                > as RecursiveCall>::__ink_TraitInfo as ::ink::reflect::TraitMessageInfo<
                                    0x2F865BD9_u32,
                                >>::PAYABLE
                            })
                            .docs([])
                            .done(),
                    ])
                    .events([
                        ::ink::metadata::EventSpec::new("CurrentValue")
                            .args([
                                ::ink::metadata::EventParamSpec::new("status")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u64,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u64"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .indexed(false)
                                    .docs([])
                                    .done(),
                            ])
                            .docs([])
                            .done(),
                    ])
                    .docs([])
                    .lang_error(
                        ::ink::metadata::TypeSpec::with_name_segs::<
                            ::ink::LangError,
                            _,
                        >(
                            ::core::iter::Iterator::map(
                                ::core::iter::IntoIterator::into_iter(["ink", "LangError"]),
                                ::core::convert::AsRef::as_ref,
                            ),
                        ),
                    )
                    .environment(
                        ::ink::metadata::EnvironmentSpec::new()
                            .account_id(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    AccountId,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["AccountId"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .balance(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Balance,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Balance"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .hash(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Hash,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Hash"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .timestamp(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Timestamp,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Timestamp"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .block_number(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    BlockNumber,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["BlockNumber"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .chain_extension(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    ChainExtension,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["ChainExtension"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .max_event_topics(MAX_EVENT_TOPICS)
                            .done(),
                    )
                    .done(),
            )
        }
    };
    use crate::rec::RecursiveCall;
}
