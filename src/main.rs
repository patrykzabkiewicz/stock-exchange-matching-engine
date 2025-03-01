use std::{cmp, ptr};

/* this particular order is not a coincidence 
   first we check if order is filled, then partially filled, 
   only then it is rested on the book*/
#[derive(Clone,Copy)]
enum OrderStatus {
	New,				// order just came in
	Filled,				// order filled completly
	PartiallyFilled,	// order filled only partially against another order
	Rested,				// order rested on the book, didn't filled at all
	Rejected,			// order was rejected after validations
	Canceled,			// order was accepted at first, rested, and then canceled
}

/* the idea behind this is that sell order is more important then buy order
   buy order only fills sell order needs and is left on the book
   so in fact we have only one loop over sell orders vector to find matching orders
   from the other side of the book */
struct SellOrder {
	id: u64,				// id number of the order
	price: u32,				// price limit of the order for matching
	volume: u32,			// total volume of the order
	concealed_volume: u32,	// concealed part of order
	volume_left: u32,		// volume that left to match later after partially matching
	filled: u32,			// already filled amount of order
	status: OrderStatus,	// current, most recent status of order
	filler: *mut BuyOrder, 	// pointer to buy order that filled this sell
}

impl SellOrder {
	
	/// returns volume filled
	fn fill(&mut self, other: &mut BuyOrder) -> u32 {
		self.filled += other.volume;
		other.filled += self.filled;
		self.volume_left -= other.volume;
		other.volume_left -= self.volume - self.volume_left;
		self.filler = other;
		self.status = OrderStatus::Filled;
		other.status = OrderStatus::Filled;
		return cmp::min(self.volume, other.volume);
	}
}


/**
	for strong typing and type safety we define a seperate type for BuyOrder
	that is slightly different than SellOrder
 */
#[derive(Clone,Copy)]
struct BuyOrder {
	id: u64,
	price: u32,
	status: OrderStatus,
	volume: u32,
	volume_left: u32,
	filled: u32,
}


#[derive(Clone,Copy)]
struct Trade {
	buyid: u64,
	sellid: u64,
	price: u32,
	volume: u32,
}


fn matching(sellvec: &mut Vec<SellOrder>, bo : &mut BuyOrder) -> Vec<Trade>
{
	// we assume that on avrage there will be no more than 5 trades from single
	// matching of orders
	let mut res: Vec<Trade> = Vec::with_capacity(5);
	
	let mut t: Trade = Trade {buyid: 0, sellid: 0, price: 0, volume: 0};
	
	for so in sellvec {
		if so.price <= bo.price {
            
            // fill the sell order
			let traded_amount = so.fill(bo);
			
			t.buyid = bo.id;
			t.sellid = so.id;
			t.price = so.price;
			t.volume = traded_amount;
			
			// push a trade to vector of trades
			res.push(t.clone());
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
		concealed_volume: 0,
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
