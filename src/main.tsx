import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import init, { initThreadPool } from "phomo-wasm";
import "./index.css";

async function initializeApp() {
  await init();
  await initThreadPool(navigator.hardwareConcurrency || 1);
}

initializeApp()
  .then(() => {
    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <App />
      </StrictMode>,
    );
  })
  .catch(console.error);
