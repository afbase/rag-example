# Rag Example Chat Application

This project is a web application built with Yew that interfaces with a locally hosted Ollama LLM to answer questions about /dev/color.

## Youtube Demo (~3 and half minutes long)

https://www.youtube.com/watch?v=Rf7OFcIKaXQ

## Prerequisites Installation

### 1. Install Ollama
```bash
# MacOS
curl https://ollama.ai/download/ollama-darwin-amd64 -o ollama
chmod +x ollama
sudo mv ollama /usr/local/bin

# Linux
curl https://ollama.ai/download/ollama-linux-amd64 -o ollama
chmod +x ollama
sudo mv ollama /usr/local/bin

# Start Ollama service
ollama serve
```

### 2. Pull and Run llama3.3 Model
```bash
# Pull the model
ollama pull llama3.3

# Test the model (optional)
ollama run llama3.3 "Hello, world!"
```

### 3. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env  # or restart your terminal
```

### 4. Install Trunk
```bash
cargo install trunk
```

### 5. Add WebAssembly Target
```bash
rustup target add wasm32-unknown-unknown
```

## Building and Running

1. Build the project:
```bash
trunk build
```

2. Configure VS Code Live Server:
   1. Open VS Code settings (Ctrl/Cmd + ,)
   2. Search for "liveServer.settings.root"
   3. Add or modify the setting:
   ```json
   {
     "liveServer.settings.root": "/dist"
   }
   ```

3. Start Live Server:
   - Click "Go Live" in the bottom right corner of VS Code
   - Or right-click the `dist` folder and select "Open with Live Server"

## Project Structure
```
rag-example/
├── Cargo.toml
├── src/
│   └── main.rs
└── dist/
    └── index.html
```

## Troubleshooting

1. If Ollama fails to start:
   - Check if the service is already running: `ps aux | grep ollama`
   - Verify port 11434 is available: `lsof -i :11434`

2. If trunk build fails:
   - Ensure all dependencies are installed: `cargo check`
   - Clean and rebuild: `trunk clean && trunk build`

3. If Live Server doesn't show the application:
   - Verify the "liveServer.settings.root" setting points to "/dist"
   - Ensure the dist folder contains the built files
   - Check browser console for any errors

## Development Tips

- During development, you can use `trunk serve` instead of Live Server for hot reloading
- The Ollama API endpoint can be modified in `main.rs` if needed
- Monitor Ollama's memory usage with `top -o %MEM` as LLMs can be resource-intensive
