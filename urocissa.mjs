import os from "os";
import { execSync } from "child_process";
import { existsSync, readdirSync, copyFileSync, rmSync } from "fs";
import { resolve, join } from "path";
import { chdir } from "process";

// ====================
// Parse Command-Line Arguments
// ====================
const args = process.argv.slice(2);
const debug = args.includes("--debug");
const update = args.includes("--update");
const install = args.includes("--install");
const run = args.includes("--run");

// ====================
// Define Project Directories
// ====================
const projectRoot = resolve();
const backendDir = join(projectRoot, "gallery-backend");
const frontendDir = join(projectRoot, "gallery-frontend");

// ====================
// Helper Functions
// ====================

/**
 * Executes a shell command synchronously.
 * @param {string} command - The command to execute.
 * @param {object} options - Options for execSync.
 */
function runCommand(command, options = {}) {
  try {
    execSync(command, { stdio: "inherit", ...options });
  } catch (error) {
    console.error(`Error executing command: ${command}`);
    process.exit(1);
  }
}

/**
 * Checks if a command exists in the system.
 * @param {string} command - The command to check.
 * @returns {boolean} - True if the command exists, false otherwise.
 */
function commandExists(command) {
  try {
    execSync(`${command}`, { stdio: "pipe" });
    return true;
  } catch {
    return false;
  }
}

/**
 * Handles errors by logging a message, returning to the root directory, and exiting.
 * @param {string} message - The error message to display.
 */
function handleError(message) {
  console.error(message);
  chdir(projectRoot);
  console.log(`Returned to the root directory: ${projectRoot}`);
  process.exit(1);
}

/**
 * Cleans up temporary FFmpeg installation files.
 * @param {string} archiveName - The name of the FFmpeg archive.
 * @param {string} extractedDir - The directory where FFmpeg was extracted.
 */
function cleanupFFmpegFiles(archiveName, extractedDir) {
  console.log("Cleaning up temporary files...");
  try {
    if (existsSync(archiveName)) {
      rmSync(archiveName); // Delete the zip file
      console.log(`${archiveName} removed.`);
    }
    if (extractedDir && existsSync(extractedDir)) {
      rmSync(extractedDir, { recursive: true, force: true }); // Delete the extracted folder
      console.log(`${extractedDir} removed.`);
    }
  } catch (cleanupError) {
    console.error(`Cleanup failed: ${cleanupError.message}`);
  }
}

/**
 * Checks if all necessary components are installed and configured.
 * @returns {boolean} - True if all checks pass, false otherwise.
 */
function runCheck() {
  let checkPassed = true;

  if (!commandExists("ffmpeg -version")) {
    console.log("FFmpeg is not installed.");
    checkPassed = false;
  }

  if (!commandExists("rustup --version")) {
    console.log("Rust is not installed.");
    checkPassed = false;
  }

  if (!existsSync(join(backendDir, ".env"))) {
    console.log(".env is not found.");
    checkPassed = false;
  }

  if (!existsSync(join(backendDir, "Rocket.toml"))) {
    console.log("Rocket.toml is not found.");
    checkPassed = false;
  }

  if (!existsSync(join(frontendDir, "config.ts"))) {
    console.log("config.ts is not found.");
    checkPassed = false;
  }

  if (!checkPassed) {
    console.log("Please run `node urocissa.mjs --install` first.");
  }

  return checkPassed;
}

// ====================
// Installation Functions
// ====================

/**
 * Installs FFmpeg based on the operating system.
 */
function installFFmpeg() {
  if (!commandExists("ffmpeg -version")) {
    console.log("FFmpeg is not installed. Installing FFmpeg...");

    if (os.platform() === "win32") {
      // Windows installation: Download and extract FFmpeg
      const ffmpegUrl =
        "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip";
      const ffmpegArchiveName = "ffmpeg.zip";
      let extractedDir;

      try {
        console.log(`Downloading FFmpeg from ${ffmpegUrl}...`);
        runCommand(`curl -L -o ${ffmpegArchiveName} ${ffmpegUrl}`);

        console.log("Extracting FFmpeg...");
        runCommand(`tar -xf ${ffmpegArchiveName}`);

        // Locate the extracted directory
        extractedDir = readdirSync(".").find(
          (dir) =>
            dir.startsWith("ffmpeg-") && dir.endsWith("-essentials_build")
        );

        if (extractedDir) {
          const ffmpegPath = resolve(join(extractedDir, "bin", "ffmpeg.exe"));
          if (existsSync(ffmpegPath)) {
            copyFileSync(ffmpegPath, "ffmpeg.exe");
            console.log("FFmpeg setup complete!");
          } else {
            console.error(
              "Failed to locate FFmpeg binary in the extracted directory."
            );
            process.exit(1);
          }
        } else {
          console.error(
            "Failed to locate the extracted FFmpeg directory. Ensure the archive was extracted properly."
          );
          process.exit(1);
        }
      } finally {
        cleanupFFmpegFiles(ffmpegArchiveName, extractedDir);
      }
    } else if (os.platform() === "linux") {
      // Linux installation: Use apt
      console.log("Installing FFmpeg using apt...");
      runCommand("sudo apt update");
      runCommand("sudo apt install -y ffmpeg");
      console.log("FFmpeg installed successfully.");
    } else {
      console.error(`Unsupported operating system: ${os.platform()}`);
      process.exit(1);
    }
  } else {
    console.log("FFmpeg is already installed.");
  }
}

/**
 * Installs Rust based on the operating system.
 */
function installRust() {
  if (!commandExists("rustup --version")) {
    console.log("Rust is not installed. Installing Rust...");

    if (os.platform() === "win32") {
      // Windows installation
      const rustInstallerUrl =
        process.arch === "x64"
          ? "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
          : "https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe";
      const installerPath = "rustup-init.exe";
      console.log(`Downloading Rust installer from ${rustInstallerUrl}...`);

      try {
        runCommand(`curl -L -o ${installerPath} ${rustInstallerUrl}`);
        console.log("Running Rust installer...");
        runCommand(installerPath);
      } finally {
        console.log("Cleaning up the installer file...");
        try {
          rmSync(installerPath);
          console.log("Installer file removed.");
        } catch (error) {
          console.error(
            `Failed to remove the installer file: ${error.message}`
          );
        }
      }
    } else if (os.platform() === "linux") {
      // Linux installation
      console.log("Installing Rust using rustup script...");
      runCommand(
        `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
      );
      console.log(
        "Rust installed successfully. You may need to restart your terminal to use Rust."
      );
    } else {
      console.error(`Unsupported operating system: ${os.platform()}`);
      process.exit(1);
    }
  } else {
    console.log("Rust is already installed.");
  }
}

/**
 * Configures and builds the backend.
 */
function configureBackend() {
  console.log("Configuring backend settings...");

  const envPath = join(backendDir, ".env");
  const envDefaultPath = join(backendDir, ".env.default");
  if (!existsSync(envPath)) {
    copyFileSync(envDefaultPath, envPath);
  }

  const rocketTomlPath = join(backendDir, "Rocket.toml");
  const rocketTomlDefaultPath = join(backendDir, "Rocket.default.toml");
  if (!existsSync(rocketTomlPath)) {
    copyFileSync(rocketTomlDefaultPath, rocketTomlPath);
  }

  console.log("Rebuilding the backend...");
  const buildCommand = debug ? "cargo build" : "cargo build --release";
  runCommand(buildCommand, { cwd: backendDir });
}

/**
 * Configures and builds the frontend.
 */
function configureFrontend() {
  console.log("Configuring frontend settings...");

  const configPath = join(frontendDir, "config.ts");
  const configDefaultPath = join(frontendDir, "config.default.ts");
  if (!existsSync(configPath)) {
    copyFileSync(configDefaultPath, configPath);
  }

  console.log("Rebuilding the frontend...");
  runCommand("npm run build", { cwd: frontendDir });
}

/**
 * Performs the installation process.
 */
function performInstall() {
  installFFmpeg();
  installRust();
  configureBackend();
  configureFrontend();

  console.log("Setup complete. Your project is ready for use!");
}

// ====================
// Update Function
// ====================

/**
 * Performs update tasks such as pulling latest changes and rebuilding.
 */
function performUpdate() {
  console.log("Update mode enabled. Performing update tasks...");

  // Step 1: Pull the latest changes
  console.log("Pulling the latest changes from the repository...");
  runCommand("git pull", { cwd: projectRoot });

  // Step 2: Rebuild the backend
  console.log("Rebuilding the backend...");
  const buildCommand = debug ? "cargo build" : "cargo build --release";
  runCommand(buildCommand, { cwd: backendDir });

  // Step 3: Rebuild the frontend
  console.log("Rebuilding the frontend...");
  runCommand("npm run build", { cwd: frontendDir });

  console.log("Update completed successfully!");
  process.exit(0);
}

// ====================
// Run Function
// ====================

/**
 * Executes the backend application.
 */
function performRun() {
  // Check prerequisites
  if (!runCheck()) {
    process.exit(1);
  }

  // Navigate to the backend directory
  try {
    console.log(`Navigating to the backend directory: ${backendDir}`);
    chdir(backendDir);
  } catch (error) {
    handleError("Failed to navigate to the backend directory.");
  }

  // Execute the backend
  try {
    const command = debug ? "cargo run" : "cargo run --release";
    console.log(`Starting the backend using '${command}'...`);
    execSync(command, { stdio: "inherit" });
    console.log("Backend is running successfully.");
  } catch (error) {
    handleError("Failed to run the backend. Check the error logs.");
  }

  // Always return to the root directory
  try {
    chdir(projectRoot);
    console.log(`Returned to the root directory: ${projectRoot}`);
  } catch (error) {
    console.error("Failed to return to the root directory.");
    process.exit(1);
  }
}

// ====================
// Main Execution Logic
// ====================
if (update) {
  performUpdate();
}

if (install) {
  performInstall();
}

if (run) {
  performRun();
}
