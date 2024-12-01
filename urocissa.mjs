import os from "os";
import { execSync } from "child_process";
import { existsSync, readdirSync, copyFileSync, rmSync } from "fs";
import { resolve, join } from "path";
import { chdir } from "process";

const args = process.argv.slice(2);
const debug = args.includes("--debug");
const update = args.includes("--update");
const install = args.includes("--install");
const run = args.includes("--run");

// Define project directories
const projectRoot = resolve();
const backendDir = join(projectRoot, "gallery-backend");
const frontendDir = join(projectRoot, "gallery-frontend");

// Helper function to execute shell commands
function runCommand(command, options = {}) {
  try {
    execSync(command, { stdio: "inherit", ...options });
  } catch (error) {
    console.error(`Error executing command: ${command}`);
    process.exit(1);
  }
}

// Helper function to check if a command exists
function commandExists(command) {
  try {
    execSync(`${command}`, { stdio: "pipe" });
    return true;
  } catch {
    return false;
  }
}

// Perform update tasks
if (update) {
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

if (install) {
  // Install FFmpeg
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
        console.log("Cleaning up temporary files...");
        try {
          if (existsSync(ffmpegArchiveName)) {
            rmSync(ffmpegArchiveName); // Delete the zip file
            console.log(`${ffmpegArchiveName} removed.`);
          }
          if (extractedDir && existsSync(extractedDir)) {
            rmSync(extractedDir, { recursive: true, force: true }); // Delete the extracted folder
            console.log(`${extractedDir} removed.`);
          }
        } catch (cleanupError) {
          console.error(`Cleanup failed: ${cleanupError.message}`);
        }
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

  // Install Rust
  if (install && !commandExists("rustup --version")) {
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

  // Configure and build the backend

  console.log("Configuring backend settings...");
  if (!existsSync(join(backendDir, ".env"))) {
    copyFileSync(join(backendDir, ".env.default"), join(backendDir, ".env"));
  }
  if (!existsSync(join(backendDir, "Rocket.toml"))) {
    copyFileSync(
      join(backendDir, "Rocket.default.toml"),
      join(backendDir, "Rocket.toml")
    );
  }
  runCommand(debug ? "cargo build" : "cargo build --release", {
    cwd: backendDir,
  });

  // Configure and build the frontend
  console.log("Configuring frontend settings...");
  if (!existsSync(join(frontendDir, "config.ts"))) {
    copyFileSync(
      join(frontendDir, "config.default.ts"),
      join(frontendDir, "config.ts")
    );
  }
  runCommand("npm run build", { cwd: frontendDir });

  console.log("Setup complete. Your project is ready for use!");
}

// Helper function to gracefully handle errors
function handleError(message) {
  console.error(message);
  chdir(projectRoot);
  console.log(`Returned to the root directory: ${projectRoot}`);
  process.exit(1);
}

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

if (run) {
  // Navigate to the backend directory
  if (!runCheck()) {
    process.exit(1);
  }

  try {
    console.log(`Navigating to the backend directory: ${backendDir}`);
    chdir(backendDir);
  } catch (error) {
    handleError("Failed to navigate to the backend directory.");
  }

  // Execute the backend
  try {
    // Determine the command based on the debug flag
    const command = debug ? "cargo run" : "cargo run --release";

    // Log the exact command that's about to run
    console.log(`Starting the backend using '${command}'...`);

    // Execute the determined command
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
