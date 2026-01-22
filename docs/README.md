# Documentation Site

This directory contains the MkDocs-based documentation site for the Totum Revolutum Bazel rules.

## Quick Start

### Option 1: Using the Shell Scripts (Recommended)

**Serve locally with live reload:**
```bash
cd docs
chmod +x serve.sh build.sh
./serve.sh
```

Then open http://127.0.0.1:8000 in your browser.

**Build static site:**
```bash
./build.sh
```

The built site will be in `docs/site/`.

### Option 2: Using Bazel

**Serve locally:**
```bash
bazel run //docs:serve
```

**Build static site:**
```bash
bazel run //docs:build
```

### Option 3: Manual Setup

```bash
cd docs

# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Serve with live reload
mkdocs serve

# Or build static site
mkdocs build
```

## Updating Documentation

### 1. Regenerate Bazel Rule Documentation

When you modify Bazel rules, regenerate their documentation:

```bash
bazel build //bazel:all_docs
```

### 2. Copy to Docs Directory

The generated Markdown files need to be copied to the docs structure:

```bash
bazel build //docs:copy_docs
```

Or manually copy from `bazel-bin/bazel/` to `docs/bazel/`.

### 3. Preview Changes

```bash
cd docs
./serve.sh
```

### 4. Build for Deployment

```bash
cd docs
./build.sh
```

## Project Structure

```
docs/
├── mkdocs.yml              # MkDocs configuration
├── requirements.txt        # Python dependencies
├── index.md               # Homepage
├── serve.sh               # Development server script
├── build.sh               # Build script
├── BUILD.bazel            # Bazel build rules
├── bazel/                 # Bazel rules documentation
│   ├── index.md          # Overview page
│   ├── dedent.md         # Generated from Stardoc
│   ├── run_copy_to_workspace.md
│   ├── sh_with_runfiles_binary.md
│   ├── rust/
│   │   ├── defs.md
│   │   ├── docs/
│   │   │   └── defs.md
│   │   └── diesel/
│   │       ├── diesel_print_schema.md
│   │       └── diesel_setup.md
│   └── containers/
│       └── postgres/
│           └── with_postgres_run.md
└── .venv/                 # Virtual environment (gitignored)
```

## Deployment Options

### GitHub Pages

1. Build the site:
   ```bash
   cd docs
   mkdocs build
   ```

2. Deploy to GitHub Pages:
   ```bash
   mkdocs gh-deploy
   ```

   This will build and push to the `gh-pages` branch.

### Manual Deployment

The `docs/site/` directory contains a complete static website that can be:
- Served by any web server (nginx, Apache, etc.)
- Deployed to Netlify, Vercel, or similar platforms
- Hosted on AWS S3, Google Cloud Storage, etc.

### CI/CD Integration

Add to your CI pipeline:

```yaml
# Example GitHub Actions workflow
- name: Build documentation
  run: |
    cd docs
    python3 -m venv .venv
    source .venv/bin/activate
    pip install -r requirements.txt
    mkdocs build

- name: Deploy to GitHub Pages
  uses: peaceiris/actions-gh-pages@v3
  with:
    github_token: ${{ secrets.GITHUB_TOKEN }}
    publish_dir: ./docs/site
```

## Customization

### Theme Configuration

Edit `mkdocs.yml` to customize:
- Colors and fonts
- Navigation structure
- Search behavior
- Social links
- Features (tabs, TOC, etc.)

See [Material for MkDocs documentation](https://squidfunk.github.io/mkdocs-material/) for all options.

### Adding Pages

1. Create a new Markdown file in the appropriate directory
2. Add it to the `nav` section in `mkdocs.yml`
3. Rebuild the site

### Custom CSS/JavaScript

Create `docs/stylesheets/extra.css` or `docs/javascripts/extra.js` and reference them in `mkdocs.yml`:

```yaml
extra_css:
  - stylesheets/extra.css
extra_javascript:
  - javascripts/extra.js
```

## Troubleshooting

### Port Already in Use

If port 8000 is already in use:
```bash
mkdocs serve -a 127.0.0.1:8001
```

### Virtual Environment Issues

Delete and recreate:
```bash
rm -rf .venv
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### Documentation Not Updating

1. Regenerate Bazel docs: `bazel build //bazel:all_docs`
2. Copy to docs: `bazel build //docs:copy_docs`
3. Clear MkDocs cache: `rm -rf docs/site`
4. Rebuild: `mkdocs build`

## Dependencies

- **Python 3.8+** - Required for MkDocs
- **MkDocs** - Static site generator
- **Material for MkDocs** - Theme
- **mkdocs-awesome-pages-plugin** - Enhanced navigation

All Python dependencies are in `requirements.txt`.

## License

Same as the main project. See [LICENSE](../LICENSE).
