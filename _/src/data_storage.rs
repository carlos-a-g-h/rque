use std::collections::HashMap;

// Group struct
pub struct Group {
	pub data: Vec<Vec<String>>
}
impl Group
{
	pub fn new() -> Group { Group { data:Vec::new() } }
	pub fn get_size(&self) -> usize { self.data.len() }
	pub fn is_empty(&self) -> bool { let size=self.get_size();if size==0 { true } else { false } }
	pub fn index_exists(&self,index:usize) -> bool { let size=self.get_size();if index>size || size==0 || size==index { false } else { true } }
	pub fn get(&self,index: usize) -> Vec<String> { if self.index_exists(index) { self.data[index].clone() } else { Vec::new() } }
	pub fn has_head(&self,head: &String) -> bool
	{
		if self.is_empty()
		{
			return false;
		};
		let mut rep=false;
		for elem in &self.data
		{
			let elem_head=elem.first().unwrap();
			if elem_head==head
			{
				rep=true;break;
			};
		};
		rep
	}
	pub fn add(&mut self,value: Vec<String>) -> bool
	{
		if value.len()==0
		{
			return false;
		};
		let val_head=value.first().unwrap();
		if self.has_head(val_head)
		{
			return false;
		};
		self.data.push(value);
		true
	}
	pub fn kick(&mut self,index: usize) -> Vec<String> { if self.index_exists(index) { self.data.remove(index) } else { Vec::new() } }
	pub fn get_range(&mut self,index: usize, qtty: usize, steal: bool) -> Vec<Vec<String>>
	{
		if !self.index_exists(index)
		{
			return Vec::new()
		};
		let size=self.get_size();
		let qtty_real:usize={ if qtty==0 { size } else { qtty } };
		let mut result:Vec<Vec<String>>=Vec::new();
		let mut pos=index;
		let mut added:usize=0;
		loop
		{
			let elem:Vec<String>={
				if steal { self.kick(pos) } else { self.get(pos) }
			};
			if elem.len()==0
			{
				break;
			};
			result.push(elem.to_vec());
			if !steal
			{
				pos=pos+1;
			};
			added=added+1;
			if pos==size || added==qtty_real
			{
				break;
			};
		};
		result
	}
}

// Main Data struct
pub struct Storage {
	pub quecol: HashMap<String,Group>
}
impl Storage
{
	pub fn new() -> Storage { Storage { quecol:HashMap::new() } }
	pub fn get_size(&self) -> usize { self.quecol.len() }
	pub fn is_empty(&self) -> bool { return self.quecol.is_empty() }
}
