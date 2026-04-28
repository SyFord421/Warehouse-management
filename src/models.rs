use serde::{Deserialize, Serialize};
use colored::*;
use serde_json;

#[derive(Serialize, Deserialize, Debug,)]
//pub biar bisa di gunakan dari luar
pub struct Item{
    pub name: String,
    pub price: f64,
    pub stock: u32,
}

impl Item {
    pub fn new(name: String, price: f64, stock: u32) -> Self {
        Self {name, price, stock}
    }
} 

#[derive(Deserialize, Serialize, Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items:Vec::new(),
        }
    }
    //Method yang Fungsi-nya untuk mengkonversi vector dan menyimpannya di dalam file json
    pub fn save_to_file(&self, filename: &str) {//mengubah vector jadi string
        match serde_json::to_string_pretty(&self.items) {
                Ok(json) => {
                    if let Err(e) = std::fs::write(filename, json){//menuliskan kedalam file dan langsung check jika gagal
                    println!("{} Gagal Menyimpan data: {}", "[×]".red(), e);
                    }else{
                    //jika berhasil
                    println!("{} Berhasil Menyimpan data", "[✓]".green());
            }
        }
        Err(e) => println!("{} Gagal Saat Mengkonversi Data: {}", "[x]".red(), e),
        }
    }
    //Method untuk memasukkan Struct kedalam Vector
    pub fn add_item(&mut self, item: Item){//move smantic memindahkan kepemilikan
        self.items.push(item);
        println!("{} Berhasil Menambahkan Barang baru", "[✓]".green());
    }
    //method untuk memuat data yang tersimpan di json
    pub fn load_from_file(filename: &str) -> Self{
    //mencheck apakah file ada jika tidak berikan vector kosong
        if let Ok(content) = std::fs::read_to_string(filename){
            match serde_json::from_str::<Vec<Item>>(&content){
                Ok(items) => {
                    println!(
                    "{} Berhasil Memuat data", "[✓]".green());
                    return Self {items};
                },
                Err(_) => {
                println!("{} Format Data Rusak", "[×]".red());
                }
            }
        }
        Self::new()
    }
    
    pub fn show_all_item(&self){//pinjam isi Vector
        for item in &self.items{
            println!(
        "Name: {} | Price: {} | Stock: {} 
        ",
        item.name,
        item.price,
        item.stock);
        }
    }
    
    
    //Method Untuk mencari item menggunakan nama
    pub fn find_item(&self, name: &str){
        let found = self
        .items
        .iter()
        .find(|i| i.name.to_lowercase() == name.to_lowercase());
        if let Some(item) = found {
            println!(
            "Name: {} | Price: {} | Stock: {}",
            item.name,
            item.price,
            item.stock
            );
        }else{
            println!("{} Barang Tidak Di Temukan", "[×]".red());
        };
    }
    
    pub fn calculate_all(&self){
        let total_value: f64 = self.items.iter().map(|item| {
            let total = item.price * item.stock as f64;
            println!("Nama: {} | Stock: {} Total: {}", item.name, item.stock, total);
            total
        }).sum();
        println!("Total {}", total_value);
    }
    //Method untuk menghapus item dari vector dengan cara menyimpan Hanya data yang tidak mirip dengan yang di cari
    pub fn remove_item(&mut self, name: &str){
        let initial_len = self.items.len();
        self.items
        .retain(|item|
        item.name.to_lowercase() != name.to_lowercase());
        if self.items.len() < initial_len {
            println!("{} Barang Berhasil Di Keluarkan", "[✓]".green());
        }else{
            println!("{} Barang Tidak di temukan", "[×]".red());
        }
    }
    
    pub fn update_or_add(&mut self, name: String, price: f64, add_stock: u32){
        let item_found = self.items
        .iter_mut()
        .find(|i| i.name.to_lowercase() == name.to_lowercase());
        match item_found {
            Some(item) => {
                item.stock += add_stock;
                println!(
                "{} Stok {} ditambah {}!  Total jadi: {}",
                "Update:".blue(),
                item.name,
                add_stock,
                item.stock);
            }, None => {
                self.items.push(Item::new(name, price, add_stock));
                println!("{} Item berhasil di tambahkan", "[✓]".green());
            }
        }
    }
    
    pub fn sell_item(&mut self, name: &str, amount: u32){
        let item_found = self.items.iter_mut().find(|i| i.name.to_lowercase() == name.to_lowercase());
        match item_found {
            Some(item) => {
                if item.stock >= amount {
                    item.stock -= amount;
                    println!("{} Berhasil menjual {} {}. Sisa stok: {}", "Success:".green(), amount, item.name, item.stock);
                    println!("TOTAL: {}", item.price * amount as f64);
                }else{
                    println!("{} Stok cuma ada {}, nggak cukup buat jual {}", "[×]".red(), item.stock, amount);
                }
            }, None => println!("{} Barang Tidak di temukan", "[×]".red()),
        }
    }
    
    pub fn check_low_stock(&self, limit: u32){
        let low_items: Vec<&Item> =
        self.items.iter().filter(|i| i.stock <= limit).collect();
        if low_items.is_empty(){
            println!("{} Stock Masih Aman ", "[✓]".green());
        }else{
            println!("--------STOCK-TIPIS --------");
            for item in low_items{
                println!("-{} Stock: {}", item.name, item.stock);
            }
        }
    }
    
}//Inventory