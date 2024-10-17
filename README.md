# Urocissa

Urocissa is a self-hosted gallery designed to serve massive collections, capable of handling millions of images and videos. It is built using Rust and Vue.

## Demo

The demo is available here: [https://demo.photoserver.tw](https://demo.photoserver.tw).
Password is `password`

This demo is currently in read-only mode, and uploading files or editing tags is not permitted at this time.

## Features

- **Blazing Fast Performance**: Search and filter millions of photos in under a second for an efficient and smooth user experience.

- **Scrubbable Timeline**: Navigate to specific dates effortlessly using the scrubbable timeline, providing precise control.

- **Infinite Timeline View**: Enjoy seamless scrolling without pagination, making browsing large collections easy.

- **Advanced Virtual Scrolling**: Urocissa has no virtual scrolling DOM height limitation of 33,554,400px. For example, see [here](https://github.com/TanStack/virtual/issues/616).

- **Instant Select All**: Select all photos instantly without needing to load the entire database.

- **Responsive Layout**: Adaptive design that allows browsing on mobile devices.

## Limitations

1. **Early Stage Development**: The app is still in its very early development phase. Many features are incomplete, and there are no automated tests.

2. **No AI Face Recognition**: The app currently does not support AI-based face recognition.

3. **Single User Only**: Only supports personal use and cannot create multiple user accounts.

[Demo](https://demo.photoserver.tw)

Experience the Urocissa gallery in action. This demo showcases the app's ability to handle large collections of photos and videos, giving you a real feel of its performance and features. Explore the interface, try the scrubbable timeline, and see how quickly you can search and filter through images.

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

   **Settings:**

   ```json
   {
     "password": "your_password_here",
     "readOnlyMode": false,
     "syncPath": ["./upload"],
     "disableImg": false,
     "discordHookUrl": null
   }
   ```

   **Explanation:**

   - `password`: Your password for the app.
   - `readOnlyMode`: Set to `false` to allow changes.
   - `syncPath`: List of directories that the app will watch for new or modified photos.
   - `disableImg`: For debugging purposes (leave it as `false`).
   - `discordHookUrl`: (Optional) Fill in your Discord webhook URL to receive error notifications.

   **Additional Settings:**

   - `port`: Default is `4000`. You can change this to your desired port number.

---

### 4. Build the Backend

Navigate to `gallery-backend` and build the backend using Cargo:

```bash
cargo run build --release
```

---

### 5. Configure Frontend Settings (Optional for Advanced Users)

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

Now, your app is configured and built!

