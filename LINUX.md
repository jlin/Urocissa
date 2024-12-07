## Steps to Set Up and Use the App (Linux Version)

Follow these steps to set up and run the Urocissa app on Linux-based systems. For instructions on setting up the app on Windows, please refer to [this guide](https://github.com/hsa00000/Urocissa/blob/main/WINDOWS.md).

### 1. Clone the Repository

```bash
git clone https://github.com/hsa00000/Urocissa.git
```

This will create a folder called `./Urocissa`.

---

### 2. Install Dependencies

Make sure the following software is installed on your system:

- **ffmpeg**: Install via your system's package manager. For Ubuntu, use APT:

  ```bash
  sudo apt update && sudo apt install -y ffmpeg
  ```

  For other Linux distributions, use the appropriate package manager (e.g., `dnf`, `yum`, `pacman`) and find the corresponding package name for installation.

- **Rust**: Install Rust using the official installer:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
  ```

- **npm (Node.js)**: Install Node.js (with npm). For Ubuntu, use APT:

  ```bash
  sudo apt install -y nodejs npm
  ```

  For other Linux distributions, use the appropriate package manager (e.g., `dnf`, `yum`, `pacman`) and find the corresponding package name for installation.

---

### 3. Configure Backend Settings

1. Navigate to the backend directory:

   ```bash
   cd ./Urocissa/gallery-backend
   ```

2. Copy the default config file and fill in the necessary settings:

   ```bash
   cp .env.default .env
   cp Rocket.default.toml Rocket.toml
   ```

   **.env:**

   ```env
   PASSWORD=password
   SYNC_PATH=./upload
   DISCORD_HOOK_URL=
   ```

   _Explanation:_

   - `PASSWORD`: Your password for the app.
   - `SYNC_PATH`: A comma-separated list of directories that the app will monitor for new or modified photos. For example: `SYNC_PATH=./upload,./some/relative/path,/some/absolute/path`. Note: `./upload` should not be removed as it is used to monitor uploaded photos and videos.
   - `DISCORD_HOOK_URL`: (Optional) Fill in your Discord webhook URL to receive error notifications.

   **Rocket.toml:**

   - `port`: Default is `5673`. You can change this to your desired port number.

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

### 6. Build the Frontend

In the `gallery-frontend` directory, run:

```bash
npm run build
```

---

### 7. Run the Application

Navigate to the `gallery-backend` directory and run the following command to start the app:

```bash
cargo run --release
```

You can now access the app via [http://127.0.0.1:5673](http://127.0.0.1:5673) or [http://127.0.0.1](http://127.0.0.1):\<your_port> if you configured a custom port in Rocket.toml.
