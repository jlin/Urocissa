
# Urocissa

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

- **ffmpeg**:  
  Install via APT on Ubuntu:

  ```bash
  sudo apt update && sudo apt install -y ffmpeg
  ```

- **Rust**:  
  Install Rust using the official installer:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
  ```

- **npm (Node.js)**:  
  Install Node.js (with npm) from NodeSource:

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
   ```

   - **`config.json` Settings:**

     ```json
     {
       "password": "your_password_here",
       "readOnlyMode": false,
       "syncPath": ["./upload"],
       "disableImg": false,
       "discordHookUrl": null
     }
     ```

   - **Explanation:**
     - `password`: Your password for the app.
     - `readOnlyMode`: Set to `false` to allow changes.
     - `syncPath`: List of directories that the app will watch for new or modified photos.
     - `disableImg`: For debugging purposes (leave it as `false`).
     - `discordHookUrl`: (Optional) Fill in your Discord webhook URL to receive error notifications.

---

### 4. Build the Backend

Navigate to `gallery-backend` and build the backend using Cargo:

```bash
cargo run build
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

   - **Note:** The `config.ts` file contains advanced settings. You can leave it unchanged unless you need to customize it.

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

Now, your app is configured and built! Follow the steps above to launch the backend and frontend successfully.
