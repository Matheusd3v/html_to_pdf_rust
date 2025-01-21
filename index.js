// const { spawn } = require("child_process");
// const path = require("path");

// // Set the path to the Rust binary
// const rustBinary = path.join(__dirname, "target/release/html_to_pdf_all");

// // Define the HTML directory
// const htmlDirectory = path.join(__dirname, "html_files"); // Change this to your actual directory

// // Spawn the Rust process
// const rustProcess = spawn(rustBinary, [htmlDirectory]);

// // Capture standard output (stdout)
// rustProcess.stdout.on("data", (data) => {
//   console.log(`Rust: ${data}`);
// });

// // Capture errors (stderr)
// rustProcess.stderr.on("data", (data) => {
//   console.error(`Rust Error: ${data}`);
// });

// // Handle process exit
// rustProcess.on("close", (code) => {
//   if (code === 0) {
//     console.log("✅ Rust script finished successfully!");
//   } else {
//     console.error(`❌ Rust script exited with code: ${code}`);
//   }
// });

const { spawn } = require("child_process");
const path = require("path");

/**
 * Executes a Rust binary with the provided HTML directory path
 * @param {string} htmlDirectoryPath - Path to the HTML files directory
 * @param {string} [rustBinaryPath] - Optional custom path to the Rust binary
 * @returns {Promise<void>} - Resolves when the process completes successfully, rejects on error
 */
async function processHtmlToPdf(htmlDirectoryPath, rustBinaryPath = null) {
  return new Promise((resolve, reject) => {
    // Set default rust binary path if not provided
    const defaultRustBinary = path.join(
      __dirname,
      "bin/html_to_pdf_all"
    );
    const finalBinaryPath = rustBinaryPath || defaultRustBinary;

    // Validate inputs
    if (!htmlDirectoryPath) {
      reject(new Error("HTML directory path is required"));
      return;
    }

    // Spawn the Rust process
    const rustProcess = spawn(finalBinaryPath, [htmlDirectoryPath]);

    // Collect stdout data
    let stdoutData = "";
    rustProcess.stdout.on("data", (data) => {
      stdoutData += data;
      console.log(`Rust: ${data}`);
    });

    // Collect stderr data
    let stderrData = "";
    rustProcess.stderr.on("data", (data) => {
      stderrData += data;
      console.error(`Rust Error: ${data}`);
    });

    // Handle process completion
    rustProcess.on("close", (code) => {
      if (code === 0) {
        console.log("✅ Rust script finished successfully!");
        resolve({ stdout: stdoutData, code });
      } else {
        const error = new Error(
          `Rust script failed with code: ${code}\nError: ${stderrData}`
        );
        error.code = code;
        error.stderr = stderrData;
        reject(error);
      }
    });

    // Handle process errors
    rustProcess.on("error", (error) => {
      reject(new Error(`Failed to start Rust process: ${error.message}`));
    });
  });
}

