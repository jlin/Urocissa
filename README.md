![螢幕擷取畫面 2024-10-17 213036](https://github.com/user-attachments/assets/b8de7937-1916-4b73-9c31-667c7eb1a23d)
# Urocissa

Urocissa is a self-hosted gallery designed to serve massive collections, capable of handling millions of images and videos. It is built using Rust and Vue.

## Motivation

The goal of this project is to efficiently serve one million photos on a 4 GB RAM server, providing smooth scrubbable scrolling, infinite photo streams, and instant search and selection, without waiting for the entire database to load in the browser.

## Demo

You can explore the features of Urocissa through the following demos:

### Standard Demo  
[https://demo.photoserver.tw](https://demo.photoserver.tw)  
**Password:** `password`  

This demo showcases the typical usage of Urocissa, allowing you to experience its core features and user interface.

### One-Million-Photo Demo  
[https://demo-million.photoserver.tw](https://demo-million.photoserver.tw)  
**Password:** `password`  

This demo demonstrates Urocissa's ability to manage 1,000,000 photos, showcasing the power and scalability of Urocissa. Since I don't have access to a million unique images, the photos in this demo are replaced with placeholders.

Both demos are currently in read-only mode, and uploading files or editing tags is not permitted at this time.

## Features

- **Blazing Fast Performance**: Indexing photos with a pure Rust crate. Instantly serve, search, and filter one million photos in under a second using an in-memory cached database.

- **Memory Efficient**: Even with the entire database cached in memory, both the standard demo and the one-million-photo demo can still run seamlessly on a single server with just 4 GB of RAM.

- **Infinite Photo Stream**: Endless scrolling without pagination. No lazy loading is needed. Urocissa uses advanced virtual scrolling to serve one million photos, overcoming the DOM height limit of 33,554,400px (see [TanStack/virtual#616](https://github.com/TanStack/virtual/issues/616)).

- **Instant Select All**: Select all photos instantly without loading the entire database.

- **Responsive Layout**: Adaptive design that allows browsing on mobile devices.

## Limitations

1. **Early Stage Development**: The app is still in its very early development phase. Many features are incomplete, and there are no automated tests. Additionally, Urocissa is currently optimized for Chrome and Firefox on Windows and Android, but it may encounter issues for browsers on iOS or Linux.

2. **No AI Face Recognition**: The app currently does not support AI-based face recognition.

3. **Single User Only**: Only supports personal use and cannot create multiple user accounts.

## Steps to Set Up and Use the App

Follow these instructions to set up and run the Urocissa app.

### 1. Clone the Repository

```bash
git clone https://github.com/hsa00000/Urocissa.git
```

This will create a folder called `./Urocissa`.

---

### 2. Install Dependencies

Make sure the following software is installed on your system:

- **ffmpeg**: Install via APT on Ubuntu:

  ```bash
  sudo apt update && sudo apt install -y ffmpeg
  ```

- **Rust**: Install Rust using the official installer:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
  ```

- **npm (Node.js)**: Install Node.js (with npm) from NodeSource:

  ```bash
  curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
  sudo apt install -y nodejs
  ```

---

### 3. Configure Backend Settings

1. Navigate to the backend directory:

   ```bash
   cd ./Urocissa/gallery-backend
   ```

2. Copy the default config file and fill in the necessary settings:

   ```bash
   cp config.default.json config.json
   cp Rocket.default.toml Rocket.toml
   ```

   **config.json:**

   ```json
   {
     "password": "your_password_here",
     "readOnlyMode": false,
     "syncPath": ["./upload"],
     "disableImg": false,
     "discordHookUrl": null
   }
   ```

   *Explanation:*

   - `password`: Your password for the app.
   - `readOnlyMode`: Set to `false` to allow changes.
   - `syncPath`: List of directories that the app will watch for new or modified photos.
   - `disableImg`: For debugging purposes (leave it as `false`).
   - `discordHookUrl`: (Optional) Fill in your Discord webhook URL to receive error notifications.

   **Rocket.toml:**

   - `port`: Default is `4000`. You can change this to your desired port number.

---

### 4. Build the Backend

Navigate to `gallery-backend` and build the backend using Cargo:

```bash
cargo build --release
```

---

### 5. Configure Frontend Settings

1. Navigate to the `gallery-frontend` directory:

   ```bash
   cd ./Urocissa/gallery-frontend
   ```

2. Copy the default frontend config file:

   ```bash
   cp config.default.ts config.ts
   ```

   **Note:** The `config.ts` file contains advanced settings. You can leave it unchanged unless you need to customize it.

---

### 6. Install Frontend Dependencies

In the `gallery-frontend` directory, run:

```bash
npm install
```

---

### 7. Build the Frontend

Build the frontend by running:

```bash
npm run build
```

---

### 8. Run the Application

Navigate to the `gallery-backend` directory and run the following command to start the app:

```bash
cargo run --release
```

You can now access the app via http://127.0.0.1:4000 or http://127.0.0.1:<your_port> if you configured a custom port in Rocket.toml.

## Update

### 1. Pull the Latest Changes from the Repository

Navigate to the project directory and pull the latest updates:

```bash
git pull
```

---

### 2. Rebuild the Frontend

1. Navigate to the `gallery-frontend` directory:

   ```bash
   cd ./Urocissa/gallery-frontend
   ```

2. Build the frontend:

   ```bash
   npm run build
   ```

---

### 3. Rebuild the Backend

1. Navigate to the `gallery-backend` directory:

   ```bash
   cd ./Urocissa/gallery-backend
   ```

2. Build and run the backend using Cargo:

   ```bash
   cargo run --release
   ```

---

After following these steps, your Urocissa app will be updated to the latest version.
