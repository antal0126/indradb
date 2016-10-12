use super::{Datastore, Transaction};
use super::test_util::*;
use traits::Id;
use models;
use std::collections::BTreeMap;
use serde_json::Value as JsonValue;
use std::marker::PhantomData;

pub struct DatastoreTestSandbox<D: Datastore<T, I>, T: Transaction<I>, I: Id> {
	pub name: String,

	pub owner_id: I,
	pub owner_secret: String,

	pub datastore: D,
	pub vertices: Vec<models::Vertex<I>>,
	pub accounts: Vec<I>,

	phantom_transaction: PhantomData<T>
}

impl<D: Datastore<T, I>, T: Transaction<I>, I: Id> DatastoreTestSandbox<D, T, I> {
	pub fn new(datastore: D) -> DatastoreTestSandbox<D, T, I> {
		return DatastoreTestSandbox{
			name: "".to_string(),
			owner_id: I::default(),
			owner_secret: "".to_string(),
			datastore: datastore,
			vertices: Vec::new(),
			accounts: Vec::new(),
			phantom_transaction: PhantomData
		};
	}

	pub fn generate_unique_string(&self, prefix: &str) -> String {
		format!("{}-{}", prefix, self.name.replace(" ", "-"))
	}

	pub fn transaction(&self) -> T {
		self.datastore.transaction(self.owner_id).unwrap()
	}

	fn search_id(&self, t: &str, name: &str) -> I {
		for vertex in self.vertices.iter() {
			if vertex.t != t {
				continue;
			}

			if vertex.properties.get("name").unwrap().as_str().unwrap().to_string() != name.to_string() {
				continue;
			}

			return vertex.id;
		}

		panic!("Could not find vertex with type=\"{}\" and name=\"{}\"", t, name);
	}

	pub fn jill_id(&self) -> I {
		self.search_id("user", "Jill")
	}

	pub fn bob_id(&self) -> I {
		self.search_id("user", "Bob")
	}

	pub fn christopher_id(&self) -> I {
		self.search_id("user", "Christopher")
	}

	pub fn memento_id(&self) -> I {
		self.search_id("movie", "Memento")
	}

	pub fn inception_id(&self) -> I {
		self.search_id("movie", "Inception")
	}

	pub fn create_test_vertex(&mut self, t: &str, name: Option<&str>) -> I {
		let trans = self.datastore.transaction(self.owner_id).unwrap();

		let props = match name {
			Some(name) => create_test_properties(name),
			None => BTreeMap::new()
		};

		let id = trans.create_vertex(t.to_string(), props.clone()).unwrap();
		trans.commit().unwrap();
		self.vertices.push(models::Vertex::new_with_properties(id, t.to_string(), props));
		id
	}

	pub fn register_account(&mut self, email: &str) -> (I, String) {
		let (id, secret) = self.datastore.create_account(email.to_string()).expect("Expected to be able to create an account");
		self.accounts.push(id);
		(id, secret)
	}

	pub fn setup(&mut self, name: &str) {
		// Set the name first
		self.name = name.to_string();

		// Create a couple of accounts
		let owner_email = self.generate_unique_string("owner");
		let (owner_id, owner_secret) = self.register_account(&owner_email[..]);
		self.owner_id = owner_id;
		self.owner_secret = owner_secret;

		// Insert some users
		let jill_id = self.create_test_vertex("user", Some("Jill"));
		let bob_id = self.create_test_vertex("user", Some("Bob"));
		let christopher_id = self.create_test_vertex("user", Some("Christopher"));

		// Insert some movies
		let doodlebug_id = self.create_test_vertex("movie", Some("Doodlebug"));
		let following_id = self.create_test_vertex("movie", Some("Following"));
		let memento_id = self.create_test_vertex("movie", Some("Memento"));
		let insomnia_id = self.create_test_vertex("movie", Some("Insomnia"));
		let batman_begins_id = self.create_test_vertex("movie", Some("Batman Begins"));
		let prestige_id = self.create_test_vertex("movie", Some("The Prestige"));
		let dark_knight_id = self.create_test_vertex("movie", Some("The Dark Knight"));
		let inception_id = self.create_test_vertex("movie", Some("Inception"));
		let dark_knight_rises_id = self.create_test_vertex("movie", Some("The Dark Knight Rises"));
		let interstellar_id = self.create_test_vertex("movie", Some("Interstellar"));

		// Create a new transaction for inserting all the test edges
		let trans = self.transaction();

		// Jill isn't a fan
		new_review_edge(&trans, jill_id, inception_id, -0.8);
		new_review_edge(&trans, jill_id, dark_knight_rises_id, -0.9);
		new_review_edge(&trans, jill_id, interstellar_id, -0.8);

		// Bob likes some stuff
		new_purchased_edge(&trans, bob_id, inception_id, 1.0);
		new_purchased_edge(&trans, bob_id, interstellar_id, 1.0);
		new_review_edge(&trans, bob_id, memento_id, 0.2);
		new_review_edge(&trans, bob_id, insomnia_id, -1.0);
		new_review_edge(&trans, bob_id, batman_begins_id, 0.7);
		new_review_edge(&trans, bob_id, prestige_id, 0.8);
		new_review_edge(&trans, bob_id, dark_knight_id, 0.9);
		new_review_edge(&trans, bob_id, inception_id, 1.0);
		new_review_edge(&trans, bob_id, dark_knight_rises_id, 0.8);
		new_review_edge(&trans, bob_id, interstellar_id, 1.0);

		// Christopher really likes these movies
		new_purchased_edge(&trans, christopher_id, doodlebug_id, 1.0);
		new_purchased_edge(&trans, christopher_id, following_id, 1.0);
		new_purchased_edge(&trans, christopher_id, memento_id, 1.0);
		new_purchased_edge(&trans, christopher_id, insomnia_id, 1.0);
		new_purchased_edge(&trans, christopher_id, batman_begins_id, 1.0);
		new_purchased_edge(&trans, christopher_id, prestige_id, 1.0);
		new_purchased_edge(&trans, christopher_id, dark_knight_id, 1.0);
		new_purchased_edge(&trans, christopher_id, inception_id, 1.0);
		new_purchased_edge(&trans, christopher_id, dark_knight_rises_id, 1.0);
		new_purchased_edge(&trans, christopher_id, interstellar_id, 1.0);
		new_review_edge(&trans, christopher_id, batman_begins_id, 1.0);
		new_review_edge(&trans, christopher_id, prestige_id, 1.0);
		new_review_edge(&trans, christopher_id, dark_knight_id, 1.0);
		new_review_edge(&trans, christopher_id, inception_id, 1.0);
		new_review_edge(&trans, christopher_id, dark_knight_rises_id, 1.0);
		new_review_edge(&trans, christopher_id, interstellar_id, 1.0);

		// Jill and Bob follow each other; Christopher is anti-social
		new_follows_edge(&trans, jill_id, bob_id, 1.0);
		new_follows_edge(&trans, bob_id, jill_id, 1.0);

		// Insert some metadata
		trans.set_metadata(None, self.generate_unique_string("global"), JsonValue::Bool(true)).unwrap();
		trans.set_metadata(Some(owner_id), self.generate_unique_string("local"), JsonValue::Bool(true)).unwrap();

		trans.commit().unwrap();
	}

	pub fn teardown(&self) {
		// Delete global metadata
		let has_global_metadata = {
			let trans = self.transaction();
			let value = trans.get_metadata(None, self.generate_unique_string("global")).is_ok();
			trans.commit().unwrap();
			value
		};

		if has_global_metadata {
			let trans = self.datastore.transaction(self.owner_id).unwrap();
			trans.delete_metadata(None, self.generate_unique_string("global")).unwrap();
			trans.commit().unwrap();
		}

		// Delete account data
		for id in self.accounts.iter() {
			self.datastore.delete_account(id.clone()).unwrap();
		}
	}
}

fn new_review_edge<T: Transaction<I>, I: Id>(trans: &T, outbound_id: I, inbound_id: I, weight: f32) {
	trans.set_edge(models::Edge::new(outbound_id, "review".to_string(), inbound_id, weight)).unwrap()
}

fn new_purchased_edge<T: Transaction<I>, I: Id>(trans: &T, outbound_id: I, inbound_id: I, weight: f32) {
	trans.set_edge(models::Edge::new(outbound_id, "purchased".to_string(), inbound_id, weight)).unwrap()
}

fn new_follows_edge<T: Transaction<I>, I: Id>(trans: &T, outbound_id: I, inbound_id: I, weight: f32) {
	trans.set_edge(models::Edge::new(outbound_id, "follows".to_string(), inbound_id, weight)).unwrap()
}
