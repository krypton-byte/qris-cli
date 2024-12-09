# qris-cli

**qris-cli** adalah sebuah alat baris perintah untuk memodifikasi data pada kode QRIS, termasuk merchant, jumlah nominal (amount), kota, dan kode pos (postal code). Alat ini dirancang untuk mempermudah pengguna dalam memanipulasi informasi pada file input QRIS dan menghasilkan file output yang diperbarui.

## Fitur

- 🚀 Mengubah informasi merchant.
- 💵 Mengubah jumlah nominal (amount).
- 🏙️ Mengubah nama kota (city).
- 📮 Mengubah kode pos (postal code).
- 📦 Output file QRIS yang diperbarui.

## Instalasi

1. Pastikan Anda telah menginstal **Rust** dan alat seperti **cargo**.
2. Clone repositori ini:
   ```bash
   git clone https://github.com/krypton-byte/qris-cli.git
   ```
3. Navigasikan ke folder proyek:
   ```bash
   cd qris-cli
   ```
4. Kompilasi dan instal:
   ```bash
   cargo build --release
   ```

Setelah berhasil, binari `qris-cli` akan tersedia di direktori `target/release`.

## Penggunaan

### Sintaks Dasar

```bash
qris-cli [OPTIONS] --input <INPUT> --output <OUTPUT>
```

### Opsi dan Parameter

| Opsi                  | Deskripsi                             | Default     |
|-----------------------|---------------------------------------|-------------|
| `-i`, `--input`       | File input QRIS.                     | Wajib       |
| `-o`, `--output`      | File output QRIS.                    | Wajib       |
| `-s`, `--size`        | Ukuran QRIS (opsional).              | 600         |
| `-m`, `--merchant`    | Nama merchant baru (opsional).       | Tidak ada   |
| `-a`, `--amount`      | Jumlah nominal baru (opsional).      | Tidak ada   |
| `-c`, `--city`        | Kota merchant baru (opsional).       | Tidak ada   |
| `-p`, `--postal-code` | Kode pos baru (opsional).            | Tidak ada   |
| `-h`, `--help`        | Menampilkan bantuan.                | -           |
| `-V`, `--version`     | Menampilkan versi aplikasi.          | -           |

### Contoh

#### Mengubah Merchant dan Nominal

```bash
qris-cli -i input.qris -o output.qris -m "Merchant Baru" -a 150000
```

#### Mengubah Kota dan Kode Pos

```bash
qris-cli -i input.qris -o output.qris -c "Bandung" -p "40123"
```

#### Menampilkan Bantuan

```bash
qris-cli --help
```

## Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

---

⭐ Jangan lupa untuk memberikan **star** pada proyek ini jika Anda merasa terbantu!
