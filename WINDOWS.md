## Steps to Set Up and Use the App (Windows Version)

Follow these instructions to set up and run the Urocissa app on a Windows machine.

### 1. Clone the Repository

First, ensure that you have Git for Windows installed. Then clone the repository using:

```bash
git clone https://github.com/hsa00000/Urocissa.git
```

This will create a folder called `./Urocissa`.

---

### 2. Install Dependencies

Make sure the following software is installed on your system:

- **ffmpeg**: Download FFmpeg from the official [FFmpeg website](https://ffmpeg.org/download.html). Extract the downloaded folder, and add the `bin` directory to your system's PATH environment variable.

- **Rust**: Install Rust using the [official installer](https://www.rust-lang.org/tools/install) for Windows.

- **Node.js (with npm)**: Download and install Node.js from the official [Node.js website](https://nodejs.org). Make sure npm is included in the installation.

---

### 3. Configure Backend Settings

1. Navigate to the backend directory:

   ```bash
   cd ./Urocissa/gallery-backend
   ```

2. Copy the default config file and fill in the necessary settings:

   ```bash
   copy .env.default .env
   copy Rocket.default.toml Rocket.toml
   ```

   **.env:**

   ```env
   PASSWORD=password
   SYNC_PATH=./upload
   DISCORD_HOOK_URL=
   ```

   _Explanation:_

   - `PASSWORD`: Your password for the app.
   - `SYNC_PATH`: List of directories that the app will watch for new or modified photos.
   - `DISCORD_HOOK_URL`: (Optional) Fill in your Discord webhook URL to receive error notifications.

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
   copy config.default.ts config.ts
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

You can now access the app via [http://127.0.0.1:4000](http://127.0.0.1:4000) or [http://127.0.0.1:<your_port>](http://127.0.0.1:<your_port>) if you configured a custom port in `Rocket.toml`.
