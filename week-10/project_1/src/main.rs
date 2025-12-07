struct Laptop {
    brand: &'static str,
    unit_price: u64,
    stock: u32,
}

impl Laptop {

    fn total_cost(&self, qty: u32) -> u64 {
        let allowed_qty = qty.min(self.stock);
        self.unit_price * allowed_qty as u64
    }
}

fn main() {
    let hp = Laptop { brand: "HP", unit_price: 650_000, stock: 10 };
    let ibm = Laptop { brand: "IBM", unit_price: 755_000, stock: 6 };
    let toshiba = Laptop { brand: "Toshiba", unit_price: 550_000, stock: 10 };
    let dell = Laptop { brand: "Dell", unit_price: 850_000, stock: 4 };

    let qty = 3;

    let total_hp = hp.total_cost(qty);
    let total_ibm = ibm.total_cost(qty);
    let total_toshiba = toshiba.total_cost(qty);
    let total_dell = dell.total_cost(qty);

    let grand_total = total_hp + total_ibm + total_toshiba + total_dell;

    println!("HP: ₦{}", total_hp);
    println!("IBM: ₦{}", total_ibm);
    println!("Toshiba: ₦{}", total_toshiba);
    println!("Dell: ₦{}", total_dell);
    println!("----------------------------");
    println!("Total cost for 3 of each brand: ₦{}", grand_total);
}
