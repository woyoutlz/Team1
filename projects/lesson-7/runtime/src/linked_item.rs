use support::{StorageMap, Parameter};
use sr_primitives::traits::Member;
use codec::{Encode, Decode};
use support::dispatch::Output;
use support::dispatch::Input;
use codec::Error;
#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]


pub struct LinkedItem<Value> {
	pub prev: Option<Value>,
	pub next: Option<Value>,
}
impl <Value>  Encode for LinkedItem<Value> where Value: Encode {
	fn encode_to<W>(&self, dest: &mut W) where W:Output {
		self.prev.encode_to(dest);
		self.next.encode_to(dest);
	}
}

impl <Value> Decode for LinkedItem<Value> where Value:Decode {
	fn decode<I>(input: &mut I) -> Result<Self, Error> where I: Input {

		Ok(LinkedItem{
			prev: match <Option<Value>>::decode(input) {
				Ok(x) => x,
				Err(e) => return Err(e),
			},
			next: match <Option<Value>>::decode(input) {
				Ok(x) => x,
				Err(e) => return Err(e),
			}
		})
	}
}

pub struct LinkedList<Storage, Key, Value>(rstd::marker::PhantomData<(Storage, Key, Value)>);

impl<Storage, Key, Value> LinkedList<Storage, Key, Value> where
    Value: Parameter + Member + Copy,
    Key: Parameter,
    Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query = Option<LinkedItem<Value>>>,
{
    fn read_head(key: &Key) -> LinkedItem<Value> {
		Self::read(key, None)
	}

	fn write_head(account: &Key, item: LinkedItem<Value>) {
		Self::write(account, None, item);
	}

	fn read(key: &Key, value: Option<Value>) -> LinkedItem<Value> {
		Storage::get(&(key.clone(), value)).unwrap_or_else(|| LinkedItem {
			prev: None,
			next: None,
		})
	}

	fn write(key: &Key, value: Option<Value>, item: LinkedItem<Value>) {
		Storage::insert(&(key.clone(), value), item);
	}

	pub fn append(key: &Key, value: Value) {
		let head = Self::read_head(key);
		let new_head = LinkedItem {
			prev: Some(value),
			next: head.next,
		};

		Self::write_head(key, new_head);

		let prev = Self::read(key, head.prev);
		let new_prev = LinkedItem {
			prev: prev.prev,
			next: Some(value),
		};
		Self::write(key, head.prev, new_prev);

		let item = LinkedItem {
			prev: head.prev,
			next: None,
		};
		Self::write(key, Some(value), item);
	}

	pub fn remove(key: &Key, value: Value) {
		if let Some(item) = Storage::take(&(key.clone(), Some(value))) {
			let prev = Self::read(key, item.prev);
			let new_prev = LinkedItem {
				prev: prev.prev,
				next: item.next,
			};

			Self::write(key, item.prev, new_prev);

			let next = Self::read(key, item.next);
			let new_next = LinkedItem {
				prev: item.prev,
				next: next.next,
			};

			Self::write(key, item.next, new_next);
		}
	}
}
