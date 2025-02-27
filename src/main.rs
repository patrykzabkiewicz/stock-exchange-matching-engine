use std::{cmp, ptr};

/* this particular order is not a coincidence 
   first we check if order is filled, then partially filled, 
   only then it is rested on the book*/
#[derive(Clone,Copy)]
enum OrderStatus {
	New,
	Filled,
	PartiallyFilled,
	Rested,
}

/* the idea behind this is that sell order is more important then buy order
   buy order only fills sell order needs and is left on the book
   so in fact we have only one loop over sell orders vector to find matching orders
   from the other side of the book */
struct SellOrder {
	id: u64,
	price: u32,
	volume: u32,
	volume_left: u32,
	filled: u32,
	status: OrderStatus,
	filler: *mut BuyOrder, // pointer to buy order that filled this sell
}

impl SellOrder {
	
	/// returns volume filled
	fn fill(&mut self, other: &mut BuyOrder) -> u32 {
		self.filled = other.volume;
		other.filled = self.filled;
		self.volume_left -= other.volume;
		other.volume -= self.volume;
		self.filler = other;
		self.status = OrderStatus::Filled;
		other.status = OrderStatus::Filled;
		return cmp::min(self.volume, other.volume);
	}
}


#[derive(Clone,Copy)]
struct BuyOrder {
	id: u64,
	price: u32,
	status: OrderStatus,
	volume: u32,
	volume_left: u32,
	filled: u32,
}


/// we need preallocated vector of trades
struct Trade {
	buyid: u64,
	sellid: u64,
	price: u32,
	volume: u32,
}


fn matching(sellvec: &mut Vec<SellOrder>, bo : &mut BuyOrder) -> Vec<Trade>
{
	let mut res: Vec<Trade> = Vec::with_capacity(5);
	
	for so in sellvec {
		if so.price <= bo.price {
            
            /// fill the sell order
			let traded_amount = so.fill(bo);
			
			// produce trade
			let t = Trade {
				buyid: bo.id,
				sellid: so.id,
				price: so.price,
				volume: traded_amount,
			};
			
			res.push(t);
		}
	}
	
	return res;
}


fn main() {
    println!("starting matching engine....");
    
    let s:SellOrder = SellOrder {
		id: 1,
		price: 33,
		volume: 1000,
		volume_left: 1000,
		status: OrderStatus::New,
		filled: 0,
		filler: ptr::null_mut(),
	};
	
	let sv : &mut Vec<SellOrder> = &mut vec![s]; // initilize vector with s order
		
    let mut b : BuyOrder = BuyOrder {
        id: 2,
        price: 33,
        volume: 1000,
        volume_left: 1000,
        filled: 0,
        status: OrderStatus::New,
    };
    
    let tv : Vec<Trade> = matching(sv, &mut b);
    
    for t in tv {
        println!("match: bid_id: {}, ask_id: {}, price: {}, volume: {}", t.buyid, t.sellid, t.price, t.volume);
    }
}
