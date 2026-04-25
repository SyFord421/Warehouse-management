use colored::*;
use serde::{Deserialize, Serialize};
use std::io::{self, Write, stdout}; //biar bias mewarnai text
use std::str::FromStr;
const FILE_DB: &str = "data.json";

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    price: f64,
    stock: u32,
}

impl Item {
    fn new(name: String, price: f64, stock: u32) -> Self {
        Self { name, price, stock }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Self {
        Self {
            //base atribut penyimpanan yang di miliki inventory agar bisa di edit di dalam trait ini
            items: Vec::new(),
        }
    }

    fn save_to_file(&self, filename: &str){
        match serde_json::to_string_pretty(&self.items){
            Ok(json) =>{
                if let Err(e) = std::fs::write(filename, json){
                    println!("{} Gagal simpan file {}", "[×]".red(), e);
                }else{
                    println!("{} Data Berhasil di simpan", "[✓]".green());
                }
            }
            Err(e) => println!("{} Gagal saat mengkonversi Data: {}", "[]".red(), e),
        }
    }
    
    //Method menambahkan item kedalam Vector
    fn add_items(&mut self, item: Item){
        self.items.push(item);
        println!("{} Item berhasil Di tambahkan", "[✓]".green());
    }
    //Method untuk menampilkan Item yang tersimpan
    fn show_all_items(&self){
        println!("----warehouse----");
        for item in &self.items {
            println!(
                "Nama: {} | Harga: {} | Stock: {}",
                item.name, item.price, item.stock
            );
        }
    }
    //Method untuk mencari Item dengan nama
    fn find_items(&self, name: &str){
        let found = self
        .items
        .iter()
        .find(|i| i.name.to_lowercase() == name.to_lowercase());
        if let Some(item) = found {
        println!(
        "{} Name: {} | Price: {} | Stock: {} |",
        "Ditemukan".green().bold(),
                item.name,
                item.price,
                item.stock
                );
            } else {
            println!("{}", "[×] Barang tidak di temukan".red());
            }
    }
    //Method Untuk menampilkan per item stock dan menjumlahkan total semuanya
    fn calculate_all(&self){
        let mut total_value: f64 = 0.0;
        for item in &self.items {
            let calculate = item.price * item.stock as f64;
            println!("Name: {} | Total Nilai: {}", item.name, calculate);
            total_value += calculate;
        }
        println!("Total Nilai Aset: {}", total_value);
    }

    fn load_from_file(filename: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string(filename){
            match serde_json::from_str::<Vec<Item>>(&content){
                Ok(items) =>{
                    println!("{} Berhasil Memuat Data", "[✓]".green());
                    return Self {items};
                },
                Err(_) => {
                    println!("{} Format Data Rusak, memulai data baru", "[!]".yellow());
                }
            }
        }
        Self::new()
    }

    //method untuk menghapus barang
    fn remove_item(&mut self, name: &str){
        //untuk menghitung total item yang tersimpan
        let initial_len = self.items.len();
        //logikanya simpan semua barang yang tidak sama dengan yang di cari
        self.items
            .retain(|item| item.name.to_lowercase() != name.to_lowercase());
        //memastikan barang sudah di keluarkan atau belum
        if self.items.len() < initial_len{
            println!("[✓] Barang Berhasil Di Keluarkan");
        }else{
            println!("[×] Yah, barang '{}' emang nggak ada dari awal 🤭", name);
        }
    }
    //method untuk update Stock dan menambahkan barang jika belum ada.
    fn update_or_add(&mut self, name: String, price: f64, add_stock: u32) {
        let item_found = self
            .items
            .iter_mut()
            .find(|i| i.name.to_lowercase() == name.to_lowercase());
        match item_found {
            Some(item) => {
                item.stock += add_stock;
                println!(
                    "{} Stok {} ditambah {}! Total jadi: {}",
                    "Update:".blue(),
                    item.name,
                    add_stock,
                    item.stock
                );
                println!("{} Jumlah stock berhasil di perbarui", "[✓]".green());
            }
            None => {
                self.items.push(Item::new(name, price, add_stock));
                println!("{}", "Barang Baru Berhasil Ditambahkan!".green());
                println!("{}", "[✓] Item berhasil di tambahkan".green());
            }
        }
    }
    
    fn sell_item(&mut self, name: &str, quantity: u32){
        let found = self.items.iter_mut().find(|i| i.name.to_lowercase() == name.to_lowercase());
        match found {
            Some(item) => {
                if item.stock >= quantity {
                    item.stock -= quantity;
                    println!("{} Berhasil menjual {} {}. Sisa stok: {}", "Success:".green(), quantity, item.name, item.stock);
                    println!("Total: {}", item.price * quantity as f64);
                    self.save_to_file(FILE_DB);
                }else{
                    println!("{} Stok cuma ada {}, nggak cukup buat jual {}", "Gagal".red(), item.stock, quantity);
                }
            },
            None => println!("{}", "[!] Barang tidak di temukan".red()),
        }
    }
    fn check_low_stock(&self, limit:u32){
            let low_items: Vec<&Item> = self.items.iter().filter(|i| i.stock <= limit).collect();
            if low_items.is_empty(){
                println!("[✓] Stock Masih Aman");
            }else{
                println!("---Stock Tipis---");
                for item in low_items{
                    println!("-{} Stock: {}", item.name, item.stock);
                }
            }
        }
    }

fn request_input(massage: &str) -> String {
    print!("{} | ", massage);
    stdout().flush().expect("Gagal melakukan flush");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("[×] Gagal Membaca input");
    input.trim().to_string()
}

fn request_number<T: FromStr>(message: &str) -> T {
    loop {
        let input = request_input(message);
        match input.parse::<T>() {
            Ok(num) => return num,
            Err(_) => println!("{}", "[!] Masukan anggka yang benar".red()),
        }
    }
}

fn main() {
    let mut warehouse = Inventory::load_from_file(FILE_DB);
    let border = "-".repeat(30);
    loop {
        println!("\n{}", border.magenta());
        println!("\n{}", "---warehouse Management---".white().bold());
        println!("\n{}", border.magenta());
        println!("{}", "[1] Tambah Barang".green());
        println!("{}", "[2] Lihat Stock".blue());
        println!("{}", "[3] Tambah atau update".white());
        println!("{}", "[4] Hapus Barang".red());
        println!("{}", "[5] Hitung Aset".yellow());
        println!("{}", "[6] Cari Barang".bright_blue());
        println!("{}", "[7] Jual Barang".green());
        println!("{}", "[8] Check Stok tipis".blue());
        println!("{}", "[9] Keluar".white());
        let choice = request_input("Masukan Pilihan");
        match choice.as_str() {
            "1" => {
                let name = request_input("Nama Barang");
                //Validasi Input agar tidak asal ketik
                let price: f64 = request_number("Masukan Harga");
                let stock: u32 = request_number("Masukan Stock");
                warehouse.add_items(Item::new(name, price, stock));
                warehouse.save_to_file(FILE_DB);
                println!("Barang Telah Di Tambahkan");
            }
            "2" => warehouse.show_all_items(),
            "3" => {
                let name = request_input("masukan nama barang");
                let price = request_number("Masukan harga barang");
                let add_stock = request_number("Masukan stock");
                warehouse.update_or_add(name, price, add_stock);
                warehouse.save_to_file(FILE_DB);
            }
            "4" => {
                let name = request_input("Nama Barang");
                warehouse.remove_item(&name);
                warehouse.save_to_file(FILE_DB);
            }
            "5" => warehouse.calculate_all(),
            "6" => {
                let name = request_input("Nama Barang");
                warehouse.find_items(&name);
            },
            "7" => {
            warehouse.show_all_items();
            let name = request_input("Nama barang");
            let qty = request_number("Jumlah barang");
            warehouse.sell_item(&name, qty);
            }
            "8" => {
                let limit = request_number("Masukan Jumlah Limit");
                warehouse.check_low_stock(limit);
            }
            "9" => break,
            _ => println!("{}", "[!] Pilihan Tidak Valid".red()),
        }
    }
}