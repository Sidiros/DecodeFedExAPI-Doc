import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";
import "./App.css";

function App() {
  const [base64Input, setBase64Input] = useState("");
  const [status, setStatus] = useState<{ type: "success" | "error"; message: string } | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const handleDecode = async () => {
    if (!base64Input.trim()) {
      setStatus({ type: "error", message: "Please enter a base64 string" });
      return;
    }

    setIsLoading(true);
    setStatus(null);

    try {
      const result = await invoke<[number[], string, string]>("decode_base64", {
        base64String: base64Input.trim(),
      });

      const [decodedData, filename, extension] = result;

      // Convert number array back to Uint8Array
      const uint8Array = new Uint8Array(decodedData);

      // Open save dialog
      const filePath = await save({
        defaultPath: filename,
        filters: [
          {
            name: extension.toUpperCase(),
            extensions: [extension],
          },
        ],
      });

      if (filePath) {
        // Write file - writeFile accepts Uint8Array for binary data
        // The save dialog automatically grants permission for the selected path
        await writeFile(filePath, uint8Array);
        setStatus({ type: "success", message: "File decoded and saved successfully!" });
        setBase64Input("");
      } else {
        setStatus({ type: "error", message: "Save dialog was cancelled" });
      }
    } catch (error) {
      let errorMessage = "Failed to decode and save file";
      if (error instanceof Error) {
        errorMessage = error.message;
      } else if (typeof error === "string") {
        errorMessage = error;
      } else if (error && typeof error === "object" && "message" in error) {
        errorMessage = String(error.message);
      }
      setStatus({
        type: "error",
        message: errorMessage,
      });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="app-container">
      <div className="card">
        <h1>FedEx Base64 Decoder</h1>
        <p className="subtitle">Decode base64 strings from FedEx Ship API</p>

        <div className="form-group">
          <label htmlFor="base64-input">Base64 String:</label>
          <textarea
            id="base64-input"
            value={base64Input}
            onChange={(e) => setBase64Input(e.target.value)}
            placeholder="Paste your base64 encoded string here..."
            rows={10}
            className="textarea-input"
          />
        </div>

        <button
          onClick={handleDecode}
          disabled={isLoading}
          className="decode-button"
        >
          {isLoading ? "Decoding..." : "Decode & Save"}
        </button>

        {status && (
          <div className={`status-message ${status.type}`}>
            {status.message}
          </div>
        )}

        <div className="info">
          <p>Supported file types: PDF, PNG, ZPLII, EPL2</p>
        </div>
      </div>
    </div>
  );
}

export default App;

