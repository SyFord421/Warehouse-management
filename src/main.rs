use colored::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{self, Write, stdout}; //biar bias mewarnai text
use std::str::FromStr;
const FILE_DB: &str = "Data.json";

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

    fn save_to_file(&self, filename: &str) {
        //Mengubah Stuct jadi string json
        let json = serde_json::to_string_pretty(&self.items).unwrap();
        //tulis ke file di memori device
        std::fs::write(filename, json).expect("Gagal menyimpan");
        println!("Berhasil menyimpan {}", filename);
    }
    //Method menambahkan item kedalam Vector
    fn add_items(&mut self, item: Item) {
        self.items.push(item);
        println!("[✓] Item berhasil Di tambahkan");
    }
    //Method untuk menampilkan Item yang tersimpan
    fn show_all_items(&self) {
        println!("----warehouse----");
        for item in &self.items {
            println!(
                "Nama: {} | Harga: {} | Stock: {}",
                item.name, item.price, item.stock
            );
        }
    }
    //Method untuk mencari Item dengan nama
    fn find_items(&self, name: &str) {
        let mut found = false;
        for item in &self.items {
            if item.name.to_lowercase() == name.to_lowercase() {
                println!(
                    "Ditemukan! Name: {} | Price: {} | Stock: {}",
                    item.name, item.price, item.stock
                );
                found = true;
                break;
            }
        }
        if !found {
            println!("[×] Barang Tidak Ditemukan");
        }
    }
    //Method Untuk menampilkan per item stock dan menjumlahkan total semuanya
    fn calculate_all(&self) {
        let mut total_value: f64 = 0.0;
        for item in &self.items {
            let calculate = item.price * item.stock as f64;
            println!("Name: {} | Total Nilai: {}", item.name, calculate);
            total_value += calculate;
        }
        println!("Total Nilai Aset: {}", total_value);
    }

    fn load_from_file(filename: &str) -> Self {
        //membaca isi menjadi string
        if let Ok(content) = std::fs::read_to_string(filename) {
            //mengubah String kembali menjadi Vector
            let items: Vec<Item> = serde_json::from_str(&content).unwrap_or(Vec::new());
            println!("[✓] Data Berhasil di muat");
            return Self { items };
        }
        println!("[!] Data masih kosong..");
        Self::new()
    }

    //method untuk menghapus barang
    fn remove_item(&mut self, name: &str) {
        //untuk menghitung total item yang tersimpan
        let initial_len = self.items.len();
        //logikanya simpan semua barang yang tidak sama dengan yang di cari
        self.items
            .retain(|item| item.name.to_lowercase() != name.to_lowercase());
        //memastikan barang sudah di keluarkan atau belum
        if self.items.len() < initial_len {
            self.save_to_file(FILE_DB);
            println!("[✓] Barang Berhasil Di Keluarkan");
        } else {
            println!("[×] Yah, barang '{}' emang nggak ada dari awal 🤭", name);
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

fn main() {
    let mut warehouse = Inventory::load_from_file(FILE_DB);
    loop {
        println!("\n{}", "===========================\n".magenta());
        println!("{}", "---warehouse Management---".white().bold());
        println!("\n{}", "===========================".magenta());
        println!("{}", "[1] Tambah Barang".green());
        println!("{}", "[2] Lihat Stock".blue());
        println!("{}", "[3] Hapus Barang".red());
        println!("{}", "[4] Hitung Aset".yellow());
        println!("{}", "[5] Cari Barang". bright_blue());
        println!("{}", "[6] Keluar".white());
        let choice = request_input("Masukan Pilihan");
        match choice.as_str() {
            "1" => {
                let name = request_input("Nama Barang");
                //Validasi Input agar tidak asal ketik
                let price: f64 = loop {
                    let p_str = request_input("Harga Barang");
                    match p_str.parse::<f64>() {
                        Ok(num) if num > 0.0 => break num,
                        _ => println!("{}", "[!] Masukan Angka Dengan benar".red()),
                    }
                };

                let stock: u32 = loop {
                    let s_int = request_input("Masukan Stock");
                    match s_int.parse::<u32>() {
                        Ok(num) => break num,
                        _ => println!("{}", "[!] Stock Harus Angka bulat".red()),
                    }
                };
                warehouse.add_items(Item::new(name, price, stock));
                warehouse.save_to_file(FILE_DB);
                println!("Barang Telah Di Tambahkan");
            }
            "2" => warehouse.show_all_items(),
            "3" => {
                let name = request_input("Nama Barang");
                warehouse.remove_item(&name);
            }
            "4" => warehouse.calculate_all(),
            "5" => {
                let name = request_input("Nama Barang");
                warehouse.find_items(&name);
            }
            "6" => break,
            _ => println!("{}", "[!] Pilihan Tidak Valid".red()),
        }
    }
}
