# Icons

This application uses Tauri's default icons. The build will work without custom icons.

## Optional: Adding Custom Icons

If you want to use custom icons instead of the default Tauri logo:

1. Create a square PNG or SVG icon (recommended: 512x512px or larger)

2. Generate all required icon formats using Tauri CLI:
```bash
npm install -g @tauri-apps/cli
tauri icon path/to/your/icon.png
```

This will generate all required icon files in this directory:
- `32x32.png` - 32x32 pixel PNG icon
- `128x128.png` - 128x128 pixel PNG icon  
- `128x128@2x.png` - 256x256 pixel PNG icon (for retina displays)
- `icon.icns` - macOS icon file
- `icon.ico` - Windows icon file

3. Update `tauri.conf.json` to include the icon paths in the `bundle.icon` array if you add custom icons.

