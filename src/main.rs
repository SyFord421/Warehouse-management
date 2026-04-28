mod models;
use models::Inventory;
use models::Item;
use colored::*;
use std::io::{self, Write, stdout}; //biar bias mewarnai text
use std::str::FromStr;
const FILE_DB: &str = "data.json";

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
            Err(_) => println!("{} Masukan anggka yang benar", "[!]".red()),
        }
    }
}

fn main() {
    let mut warehouse = Inventory::load_from_file(FILE_DB);
    let border = "-".repeat(30);
    loop {
        println!("\n{}", border.magenta());
        println!("\n{}", "---Warehouse Management---".white().bold());
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
                warehouse.add_item(Item::new(name, price, stock));
                warehouse.save_to_file(FILE_DB);
                println!("Barang Telah Di Tambahkan");
            }
            "2" => warehouse.show_all_item(),
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
                warehouse.find_item(&name);
            },
            "7" => {
            warehouse.show_all_item();
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