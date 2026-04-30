use serde::{Deserialize, Serialize};
use colored::*;
use serde_json;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;//untuk mengambil waktu lokal

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
    pub items: HashMap<String, Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items:HashMap::new(),
        }
    }
    pub fn log_transaction(&self, massage: &str){
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!("[{}] {}\n", now, massage);
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("history.txt")
        .expect("Gagal Membuka File Log");
        file.write_all(log_entry.as_bytes()).expect("Gagal Menuliskan Log");
    }
    
    
    pub fn save_to_file(&self, filename: &str) {
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
        self.items.insert(item.name.clone(), item);
        println!("{} Berhasil Menambahkan Barang baru", "[✓]".green());
    }
    //method untuk memuat data yang tersimpan di json
    pub fn load_from_file(filename: &str) -> Self{
    //mencheck apakah file ada jika tidak berikan vector kosong
        if let Ok(content) = std::fs::read_to_string(filename){
            match serde_json::from_str::<HashMap<String, Item>>(&content){
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
    
    pub fn show_all_item(&self) {
    // Di HashMap, kita dapet kuncinya (nama) dan isinya (item)
    for (name, item) in &self.items {
        println!("Name: {} | Price: {} | Stock: {}", name, item.price, item.stock);
        }
    }

    
    
    //Method Untuk mencari item menggunakan nama
    pub fn find_item(&self, name: &str){
        if let Some(item) = self.items.get(name) {
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
    let total_value: f64 = self.items.values().map(|item| {
        let total = item.price * item.stock as f64;
        println!("Nama: {} | Stock: {} Subtotal: {}", item.name, item.stock, total);
        total
    }).sum();
    println!("{} Total Nilai Value: {}", ">>".yellow(), total_value);
    }
    
    pub fn remove_item(&mut self, name: &str){
        if self.items.remove(name).is_some(){
            println!("{} Barang Berhasil Di hapus", "[✓]".green());
        }else{
            println!("{} Barang Tidak Di Temukan", "[×]".red());
        }
    }
    
    pub fn update_or_add(&mut self, name: String, price: f64, add_stock: u32){
        let mut is_updated = false;
        if let Some(item) = self.items.get_mut(&name) {
            item.stock += add_stock;
            println!("{} Stok {} ditambah {}!", "Update:".blue(), item.name, add_stock);
            is_updated = true;
            }//nyerah gw mending pake flag sedekah 1 perak nggak bikin rugi
        if is_updated{
            self.log_transaction(&format!("Update stok {}: +{}", name, add_stock));
        }else{
            let new_item = Item::new(name.clone(), price, add_stock);
            self.items.insert(name, new_item);
            println!("{} Barang Telah Ditambahkan", "[✓]".green());
        }
    }
    
    pub fn sell_item(&mut self, name: &str, amount: u32){
        match self.items.get_mut(name){
            Some(item) => {
                if item.stock >= amount{
                    item.stock -= amount;
                    println!("{} Berhasil menjual {} {}. Sisa stok: {}", "Success:".green(), amount, item.name, item.stock);
                    let massage = format!("Jual {} sebanyak {} unit", item.name, amount);
                    self.log_transaction(&massage);
                }else{
                    println!("{} Stok hanya ada {}, Tidak Cukup!", "[×]".red(), item.stock);
                }
            },
            None => println!("{} Barang tidak ditemukan!", "[×]".red()),
        }
    }
    
    pub fn check_low_stock(&self, limit: u32){
        let low_items: Vec<&Item> = self.items.values().filter(|i| i.stock <= limit).collect();
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