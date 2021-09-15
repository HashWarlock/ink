// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::marker::PhantomData;

/// Used to refer to the generated contract reference from the given contract storage struct.
pub trait ContractReference {
    /// The generated contract reference type.
    type Type;
}

/// Used to refer to the generated contract call builder.
///
/// The generated contract call builder implements the long-hand calling API
/// for all inherent or trait ink! messages.
pub trait ContractCallBuilder {
    /// The generated contract call builder type.
    type Type;
}

/// Trait to inform about the name of an ink! smart contract.
pub trait ContractName {
    /// The name of the ink! smart contract.
    const NAME: &'static str;
}

/// A generic ink! smart contract call builder.
///
/// This utility struct is generic over the ink! environment `E`
/// as well as over a `T`, usually a concrete smart contract.
///
/// This is used by the ink! codegen in order to implement various
/// implementations for calling smart contract instances of contract
/// `T` using environment `E` on-chain.
#[repr(transparent)]
pub struct CallBuilderBase<T, E>
where
    E: ink_env::Environment,
{
    account_id: <E as ink_env::Environment>::AccountId,
    __marker: PhantomData<fn() -> T>,
}

impl<T, E> core::fmt::Debug for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: core::fmt::Debug,
    T: ContractName,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = <T as ContractName>::NAME;
        f.debug_struct("ContractRef")
            .field("name", &name)
            .field("account_id", &self.account_id)
            .finish()
    }
}

impl<T, E> Copy for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Copy,
{
}

impl<T, E> Clone for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            account_id: self.account_id.clone(),
            __marker: PhantomData,
        }
    }
}

impl<T, E> scale::Encode for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: scale::Encode,
{
    #[inline]
    fn size_hint(&self) -> usize {
        <<E as ink_env::Environment>::AccountId as scale::Encode>::size_hint(
            &self.account_id,
        )
    }

    #[inline]
    fn encode_to<O: scale::Output + ?Sized>(&self, dest: &mut O) {
        <<E as ink_env::Environment>::AccountId as scale::Encode>::encode_to(
            &self.account_id,
            dest,
        )
    }
}

impl<T, E> scale::Decode for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: scale::Decode,
{
    #[inline]
    fn decode<I: scale::Input>(input: &mut I) -> Result<Self, scale::Error> {
        <<E as ink_env::Environment>::AccountId as scale::Decode>::decode(input)
            .map(<Self as ink_env::call::FromAccountId<E>>::from_account_id)
    }
}

impl<T, E> ink_env::call::FromAccountId<E> for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
{
    #[inline]
    fn from_account_id(account_id: <E as ink_env::Environment>::AccountId) -> Self {
        Self {
            account_id,
            __marker: PhantomData,
        }
    }
}

impl<T, E> crate::ToAccountId<E> for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Clone,
{
    #[inline]
    fn to_account_id(&self) -> <E as ink_env::Environment>::AccountId {
        <<E as ink_env::Environment>::AccountId as core::clone::Clone>::clone(
            &self.account_id,
        )
    }
}

impl<T, E> core::hash::Hash for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: core::hash::Hash,
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: core::hash::Hasher,
    {
        <<E as ink_env::Environment>::AccountId as core::hash::Hash>::hash(
            &self.account_id,
            state,
        )
    }
}

impl<T, E> core::cmp::PartialEq for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: core::cmp::PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.account_id == other.account_id
    }
}

impl<T, E> core::cmp::Eq for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Eq,
{
}

impl<T, E> ink_storage::traits::SpreadLayout for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: ink_storage::traits::SpreadLayout,
{
    const FOOTPRINT: u64 = 1;
    const REQUIRES_DEEP_CLEAN_UP: bool = false;

    #[inline]
    fn pull_spread(ptr: &mut ::ink_primitives::KeyPtr) -> Self {
        Self {
            account_id: <<E as ink_env::Environment>::AccountId
                as ink_storage::traits::SpreadLayout>::pull_spread(ptr),
            __marker: PhantomData,
        }
    }

    #[inline]
    fn push_spread(&self, ptr: &mut ::ink_primitives::KeyPtr) {
        <<E as ink_env::Environment>::AccountId
            as ink_storage::traits::SpreadLayout>::push_spread(&self.account_id, ptr)
    }

    #[inline]
    fn clear_spread(&self, ptr: &mut ::ink_primitives::KeyPtr) {
        <<E as ink_env::Environment>::AccountId
            as ink_storage::traits::SpreadLayout>::clear_spread(&self.account_id, ptr)
    }
}

impl<T, E> ink_storage::traits::PackedLayout for CallBuilderBase<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: ink_storage::traits::PackedLayout,
{
    #[inline(always)]
    fn pull_packed(&mut self, _at: &ink_primitives::Key) {}

    #[inline(always)]
    fn push_packed(&self, _at: &ink_primitives::Key) {}

    #[inline(always)]
    fn clear_packed(&self, _at: &ink_primitives::Key) {}
}

/// A generic ink! smart contract call forwader.
///
/// A call forwarder is a thin wrapper around a call builder
/// that forwards the long-hand calls to the builder and directly
/// serves as the interface for the short-hand calls and constructors.
///
/// This utility struct is generic over the ink! environment `E`
/// as well as over a `T`, usually a concrete smart contract.
///
/// This is used by the ink! codegen in order to implement various
/// implementations for calling smart contract instances of contract
/// `T` using environment `E` on-chain.
#[repr(transparent)]
pub struct ContractRef<T, E>
where
    E: ink_env::Environment,
{
    call_builder: CallBuilderBase<T, E>,
}

impl<T, E> crate::TraitCallBuilder for ContractRef<T, E>
where
    E: ink_env::Environment,
{
    type Builder = CallBuilderBase<T, E>;

    #[inline]
    fn call(&self) -> &Self::Builder {
        &self.call_builder
    }

    #[inline]
    fn call_mut(&mut self) -> &mut Self::Builder {
        &mut self.call_builder
    }
}

impl<T, E> core::fmt::Debug for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: core::fmt::Debug,
    T: ContractName,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CallForwarderBase")
            .field("call_builder", &self.call_builder)
            .finish()
    }
}

impl<T, E> Copy for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Copy,
{
}

impl<T, E> Clone for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            call_builder: <CallBuilderBase<T, E> as Clone>::clone(&self.call_builder),
        }
    }
}

impl<T, E> scale::Encode for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: scale::Encode,
{
    #[inline]
    fn size_hint(&self) -> usize {
        <CallBuilderBase<T, E> as scale::Encode>::size_hint(&self.call_builder)
    }

    #[inline]
    fn encode_to<O: scale::Output + ?Sized>(&self, dest: &mut O) {
        <CallBuilderBase<T, E> as scale::Encode>::encode_to(&self.call_builder, dest)
    }
}

impl<T, E> scale::Decode for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: scale::Decode,
{
    #[inline]
    fn decode<I: scale::Input>(input: &mut I) -> Result<Self, scale::Error> {
        <CallBuilderBase<T, E> as scale::Decode>::decode(input)
            .map(|call_builder| Self { call_builder })
    }
}

impl<T, E> ink_env::call::FromAccountId<E> for ContractRef<T, E>
where
    E: ink_env::Environment,
{
    #[inline]
    fn from_account_id(account_id: <E as ink_env::Environment>::AccountId) -> Self {
        Self {
            call_builder: <CallBuilderBase<T, E>
                as ink_env::call::FromAccountId<E>>::from_account_id(account_id)
        }
    }
}

impl<T, E> crate::ToAccountId<E> for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Clone,
{
    #[inline]
    fn to_account_id(&self) -> <E as ink_env::Environment>::AccountId {
        <CallBuilderBase<T, E> as crate::ToAccountId<E>>::to_account_id(
            &self.call_builder,
        )
    }
}

impl<T, E> core::hash::Hash for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: core::hash::Hash,
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: core::hash::Hasher,
    {
        <CallBuilderBase<T, E> as core::hash::Hash>::hash(&self.call_builder, state)
    }
}

impl<T, E> core::cmp::PartialEq for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: core::cmp::PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.call_builder == other.call_builder
    }
}

impl<T, E> core::cmp::Eq for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: Eq,
{
}

impl<T, E> ink_storage::traits::SpreadLayout for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: ink_storage::traits::SpreadLayout,
{
    const FOOTPRINT: u64 = 1;
    const REQUIRES_DEEP_CLEAN_UP: bool = false;

    #[inline]
    fn pull_spread(ptr: &mut ::ink_primitives::KeyPtr) -> Self {
        Self {
            call_builder:
                <CallBuilderBase<T, E> as ink_storage::traits::SpreadLayout>::pull_spread(
                    ptr,
                ),
        }
    }

    #[inline]
    fn push_spread(&self, ptr: &mut ::ink_primitives::KeyPtr) {
        <CallBuilderBase<T, E> as ink_storage::traits::SpreadLayout>::push_spread(
            &self.call_builder,
            ptr,
        )
    }

    #[inline]
    fn clear_spread(&self, ptr: &mut ::ink_primitives::KeyPtr) {
        <CallBuilderBase<T, E> as ink_storage::traits::SpreadLayout>::clear_spread(
            &self.call_builder,
            ptr,
        )
    }
}

impl<T, E> ink_storage::traits::PackedLayout for ContractRef<T, E>
where
    E: ink_env::Environment,
    <E as ink_env::Environment>::AccountId: ink_storage::traits::PackedLayout,
{
    #[inline(always)]
    fn pull_packed(&mut self, _at: &ink_primitives::Key) {}

    #[inline(always)]
    fn push_packed(&self, _at: &ink_primitives::Key) {}

    #[inline(always)]
    fn clear_packed(&self, _at: &ink_primitives::Key) {}
}
