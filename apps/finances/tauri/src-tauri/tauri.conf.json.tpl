{
  "app": {
    "security": {
      "csp": null
    },
    "windows": [
      {
        "height": 1500,
        "title": "finances",
        "width": 1500
      }
    ]
  },
  "build": {
    "beforeBuildCommand": "pnpm build",
    "beforeDevCommand": "pnpm dev",
    "devUrl": "http://localhost:1420",
    "frontendDist": "%FRONTEND_DIST%"
  },
  "bundle": {
    "active": true,
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "targets": "all"
  },
  "identifier": "com.finances.app",
  "productName": "finances",
  "version": "0.1.0"
}
