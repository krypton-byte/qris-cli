use std::process::exit;

use clap::{Parser, ValueHint};
use fast_qr::convert::image::ImageBuilder;
use qris::node::Nodes;
use image;
use rqrr;
use fast_qr::qr::QRBuilder;
use fast_qr::convert::{Builder, Shape};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[clap(value_hint = ValueHint::FilePath)]
    #[arg(short, long)]
    input: String,
    #[clap(value_hint = ValueHint::DirPath)]
    #[arg(short, long)]
    output: String,
    #[arg(short, long, default_value_t = 600)]
    size: u32,
    #[arg(short, long)]
    merchant: Option<String>,
    #[arg(short, long)]
    amount: Option<usize>,
    #[arg(short, long)]
    city: Option<String>,
    #[arg(short, long)]
    postal_code: Option<String>,
}

fn main() {
    let parser = Args::parse();
    let img = image::open(parser.input).unwrap().to_luma8();
    let mut prepare_img = rqrr::PreparedImage::prepare(img);
    let grids = prepare_img.detect_grids();
    if grids.len() == 0{
        eprintln!("QR Object Not Detected");
        exit(1);
    }
    match grids[0].decode(){
        Ok((_, content))=>{
            let mut nodes = Nodes::from_str(&content).unwrap();
            if let Some(merchant) = parser.merchant {
                nodes.set_merchant_name(merchant);
            }
            if let Some(amount) = parser.amount{
                nodes.set_amount(amount);
            }
            if let Some(city) = parser.city {
                nodes.set_merchant_city(city);
            }
            if let Some(code) = parser.postal_code {
                nodes.set_postal_code(code);
            }
            //update checksum
            nodes.rewrite_crc16();
            let qr_modified = nodes.dumps();
            let qrcode = QRBuilder::new(qr_modified).build().unwrap();
            let result = ImageBuilder::default().shape(Shape::Square).fit_width(parser.size).to_file(&qrcode, &parser.output);
            match result {
                Ok(_)=> {
                    println!("qr saved as {}", parser.output);
                },
                Err(e)=>{
                    println!("generate qr failure: {}", e);
                }
            }
        },
        Err(r)=>{
            eprint!("Error: {}", r);
            exit(1);
        }
    }
}
